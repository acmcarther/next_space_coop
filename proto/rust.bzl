load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_doc",
    "rust_doc_test",
)

load("@org_pubref_rules_protobuf//protobuf:rules.bzl", "proto_compile")
load("//tools/rust:codegen.bzl", "gen_rust_library")

def rust_proto_library(
    name,
    protos = [],
    srcs = [],
    proto_deps = [],
    proto_dep_crates = []):

  extern_crates = ["extern crate protobuf;"] + ["extern crate {};".format(dep) for dep in proto_dep_crates]

  proto_dep_uses = ["pub use {}::*;".format(dep) for dep in proto_dep_crates]
  mod_names = ["pub mod {};".format(p_name.split(".")[0]) for p_name in protos];
  lib_rs_content = "\n".join(extern_crates) + "\n".join(proto_dep_uses) + "\n".join(mod_names)

  proto_compile(
    name = name + ".pb",
    # Pass in a list of proto_language rules
    langs = ["//proto:rust"],
    deps = [dep + ".pb" for dep in proto_deps],
    protos = protos
  )

  native.genrule(
    name = name + "lib_rs",
    outs = ["lib.rs"],
    # This is a pretty naive soln
    cmd = "echo \"" + lib_rs_content + "\" > $@"
  )

  rust_library(
    name = name,
    srcs = [name + "lib_rs"] + [name + ".pb"],
    deps = ["@protoc_gen_rust//:protobuf",] + proto_deps,
  )
