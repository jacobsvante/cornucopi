<div align="center"> <img src="https://raw.githubusercontent.com/jacobsvante/cornucopi/main/assets/logo.svg" width=200 /> </div>
<h1 align="center">Cornucopi</h1>
<div align="center">
 <strong>
   Generate type-checked Rust from your SQL
 </strong>
<p>
   NOTE: It's been a long time since the original `cornucopia` crates were updated. These are copies of those crates under a new name, where we will focus on keeping sub-dependencies up-to-date.
 </p>
</div>

<br />

<div align="center">
  <!-- Version -->
  <a href="https://crates.io/crates/cornucopi_sync">
    <img src="https://img.shields.io/crates/v/cornucopi_sync.svg?style=flat-square"
    alt="Crates.io version" />
  </a>

  <!-- Book -->
  <a href="https://cornucopi-rs.netlify.app/book/index.html">
  <img src="https://img.shields.io/badge/book-latest-blue?logo=mdbook&style=flat-square" alt="book">
  </a>

  <!-- Docs -->
  <a href="https://docs.rs/cornucopi_sync/latest/cornucopi_sync/">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/cornucopi_sync?style=flat-square">
  </a>

  <!-- License -->
  <a href="https://github.com/jacobsvante/cornucopi#License">
    <img src="https://img.shields.io/badge/License-APACHE--2.0%2FMIT-blue?style=flat-square" alt="License">
  </a>

  <!-- Chat -->
  <a href="https://discord.gg/nYwUmQDHBZ">
    <img src="https://img.shields.io/discord/987088069280825401?label=chat&logo=discord&style=flat-square" alt="Chat">
  </a>
</div>

---

**Note:** This crate is the *synchronous* client. You can find the *asynchronous* client [here](https://crates.io/crates/cornucopi_async).

This is a client crate for [Cornucopi](https://crates.io/crates/cornucopi). This dependency provides
1. Internals required by the generated code.
2. Public items that you may find useful when working with Cornucopi (you can find more info about these in the [docs](https://docs.rs/cornucopi_sync/latest/cornucopi_sync/)).

***You need to depend on this crate for Cornucopi's generated code to work properly.***