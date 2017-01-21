macro_rules! remove_overriden {
    (@remove $_self:ident, $v:ident, $a:ident.$ov:ident) => {
        if let Some(ref ora) = $a.$ov() {
            vec_remove_all!($_self.$v, ora);
        }
    };
    (@arg $_self:ident, $arg:ident) => {
        remove_overriden!(@remove $_self, required, $arg.requires);
        remove_overriden!(@remove $_self, blacklist, $arg.blacklist);
        remove_overriden!(@remove $_self, overrides, $arg.overrides);
    };
    ($_self:ident, $name:expr) => {
        debugln!("macro=remove_overriden!;");
        if let Some(ref o) = $_self.opts.iter().filter(|o| o.b.name == *$name).next() {
            remove_overriden!(@arg $_self, o);
        } else if let Some(ref f) = $_self.flags.iter().filter(|f| f.b.name == *$name).next() {
            remove_overriden!(@arg $_self, f);
        } else if let Some(p) = $_self.positionals.values().filter(|p| p.b.name == *$name).next() {
            remove_overriden!(@arg $_self, p);
        }
    };
}

macro_rules! arg_post_processing {
    ($me:ident, $arg:ident, $matcher:ident) => {
        debugln!("macro=arg_post_processing!;");
        // Handle POSIX overrides
        debug!("Is '{}' in overrides...", $arg.to_string());
        if $me.overrides.contains(&$arg.name()) {
            if let Some(ref name) = find_name_from!($me, &$arg.name(), overrides, $matcher) {
                sdebugln!("Yes by {}", name);
                $matcher.remove(name);
                remove_overriden!($me, name);
            }
        } else { sdebugln!("No"); }

        // Add overrides
        debug!("Does '{}' have overrides...", $arg.to_string());
        if let Some(or) = $arg.overrides() {
            sdebugln!("Yes");
            $matcher.remove_all(or);
            for pa in or { remove_overriden!($me, pa); }
            $me.overrides.extend(or);
            vec_remove_all!($me.required, or);
        } else { sdebugln!("No"); }

        // Handle conflicts
        debug!("Does '{}' have conflicts...", $arg.to_string());
        if let Some(bl) = $arg.blacklist() {
            sdebugln!("Yes");
            
            for c in bl {
                // Inject two-way conflicts
                debug!("Has '{}' already been matched...", c);
                if $matcher.contains(c) {
                    sdebugln!("Yes");
                    // find who blacklisted us...
                    $me.blacklist.push(&$arg.b.name);
                } else {
                    sdebugln!("No");
                }
            }

            $me.blacklist.extend(bl);
            vec_remove_all!($me.overrides, bl);
            vec_remove_all!($me.required, bl);
        } else { sdebugln!("No"); }

        // Add all required args which aren't already found in matcher to the master
        // list
        debug!("Does '{}' have requirements...", $arg.to_string());
        if let Some(reqs) = $arg.requires() {
            for n in reqs {
                if $matcher.contains(&n) {
                    sdebugln!("\tYes '{}' but it's already met", n);
                    continue;
                } else { sdebugln!("\tYes '{}'", n); }

                $me.required.push(n);
            }
        } else { sdebugln!("No"); }

        _handle_group_reqs!($me, $arg);
    };
}

macro_rules! _handle_group_reqs{
    ($me:ident, $arg:ident) => ({
        use args::AnyArg;
        debugln!("macro=_handle_group_reqs!;");
        for grp in $me.groups.values() {
            let found = if grp.args.contains(&$arg.name()) {
                vec_remove!($me.required, &$arg.name());
                if let Some(ref reqs) = grp.requires {
                    $me.required.extend(reqs);
                }
                if let Some(ref bl) = grp.conflicts {
                    $me.blacklist.extend(bl);
                }
                true // What if arg is in more than one group with different reqs?
            } else {
                false
            };
            debugln!("iter;grp={};found={:?}", grp.name, found);
            if found {
                vec_remove_all!($me.required, &grp.args);
                debugln!("Adding args from group to blacklist...{:?}", grp.args);
                if !grp.multiple {
                    $me.blacklist.extend(&grp.args);
                    vec_remove!($me.blacklist, &$arg.name());
                }
            }
        }
    })
}

macro_rules! validate_multiples {
    ($_self:ident, $a:ident, $m:ident) => {
        debugln!("macro=validate_multiples!;");
        if $m.contains(&$a.b.name) && !$a.b.settings.is_set(ArgSettings::Multiple) {
            // Not the first time, and we don't allow multiples
            return Err(Error::unexpected_multiple_usage($a,
                &*$_self.create_current_usage($m),
                $_self.color()))
        }
    };
}

macro_rules! parse_positional {
    (
        $_self:ident,
        $p:ident,
        $arg_os:ident,
        $pos_counter:ident,
        $matcher:ident
    ) => {
        debugln!("macro=parse_positional!;");
        validate_multiples!($_self, $p, $matcher);

        if !$_self.trailing_vals &&
           ($_self.settings.is_set(AppSettings::TrailingVarArg) &&
            $pos_counter == $_self.positionals.len()) {
            $_self.trailing_vals = true;
        }
        let _ = try!($_self.add_val_to_arg($p, &$arg_os, $matcher));

        $matcher.inc_occurrence_of($p.b.name);
        let _ = $_self.groups_for_arg($p.b.name)
                      .and_then(|vec| Some($matcher.inc_occurrences_of(&*vec)));
        arg_post_processing!($_self, $p, $matcher);
        // Only increment the positional counter if it doesn't allow multiples
        if !$p.b.settings.is_set(ArgSettings::Multiple) {
            $pos_counter += 1;
        }
    };
}

macro_rules! find_from {
    ($_self:ident, $arg_name:expr, $from:ident, $matcher:expr) => {{
        let mut ret = None;
        for k in $matcher.arg_names() {
            if let Some(f) = find_by_name!($_self, &k, flags, iter) {
                if let Some(ref v) = f.$from() {
                    if v.contains($arg_name) {
                        ret = Some(f.to_string());
                    }
                }
            }
            if let Some(o) = find_by_name!($_self, &k, opts, iter) {
                if let Some(ref v) = o.$from() {
                    if v.contains(&$arg_name) {
                        ret = Some(o.to_string());
                    }
                }
            }
            if let Some(pos) = find_by_name!($_self, &k, positionals, values) {
                if let Some(ref v) = pos.$from() {
                    if v.contains($arg_name) {
                        ret = Some(pos.b.name.to_owned());
                    }
                }
            }
        }
        ret
    }};
}

macro_rules! find_name_from {
    ($_self:ident, $arg_name:expr, $from:ident, $matcher:expr) => {{
        let mut ret = None;
        for k in $matcher.arg_names() {
            if let Some(f) = find_by_name!($_self, &k, flags, iter) {
                if let Some(ref v) = f.$from() {
                    if v.contains($arg_name) {
                        ret = Some(f.b.name);
                    }
                }
            }
            if let Some(o) = find_by_name!($_self, &k, opts, iter) {
                if let Some(ref v) = o.$from() {
                    if v.contains(&$arg_name) {
                        ret = Some(o.b.name);
                    }
                }
            }
            if let Some(pos) = find_by_name!($_self, &k, positionals, values) {
                if let Some(ref v) = pos.$from() {
                    if v.contains($arg_name) {
                        ret = Some(pos.b.name);
                    }
                }
            }
        }
        ret
    }};
}

// Finds an arg by name
macro_rules! find_by_name {
    ($_self:ident, $name:expr, $what:ident, $how:ident) => {
        $_self.$what.$how().find(|o| &o.b.name == $name)
    }
}

// Finds an option including if it's aliasesed
macro_rules! find_by_long {
    ($_self:ident, $long:expr, $what:ident) => {
        $_self.$what
            .iter()
            .filter(|o| o.s.long.is_some())
            .find(|o| {
                &&o.s.long.unwrap() == &$long ||
                (o.s.aliases.is_some() &&
                 o.s
                    .aliases
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|&(alias, _)| &&alias == &$long))
            })
    }
}
