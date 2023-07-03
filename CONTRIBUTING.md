# Contributing to ovhdata-cli
 
This project accepts contributions. In order to contribute, you should
pay attention to a few things:
 
1. your code must follow the coding style rules
2. your code must be unit-tested
3. your code must be documented
4. your work must be signed (see below)
5. you may contribute through GitHub Pull Requests
 
# Coding and documentation Style
 
This is a set of recommendations on how to design and present APIs for the Rust programming language. They are authored largely by the Rust library team, based on experiences building the Rust standard library and other crates in the Rust ecosystem.

Read them [here](https://github.com/rust-lang/rustfmt)

## Code Style
The code linter used is `cargo clippy`, all warning have to be fixed before merge a PR.

## Commit titles 
All commit titles must respect the following format.  Warning, the first commits in this project did not have this rule, please don't copy them. 

```[ISSUE][COMMIT_TYPE] Message```

The issue may be optional (build and release commits for example).
Please use the following commit types.

* feat – a new feature is introduced with the changes
* fix – a bug fix has occurred
* core – changes that do not relate to a fix or feature and don't modify src or test files (for example updating dependencies)
* refactor – refactored code that neither fixes a bug nor adds a feature
* docs – updates to documentation such as a the README or other markdown files
* style – changes that do not affect the meaning of the code, likely related to code formatting such as white-space, missing semi-colons, and so on.
* test – including new or correcting previous tests
* perf – performance improvements
* ci – continuous integration related
* build – changes that affect the build system or external dependencies
* revert – reverts a previous commit

# Submitting Modifications
 
The contributions should be submitted through Github Pull Requests and follow the DCO which is defined below.
Before merge, any PR must be squashed with a single commit per commit type.  
 
# Licensing for new files
 
ovhdata-cli is licensed under a Apache 2 license. Anything
contributed to ovhdata-cli must be released under this license.
 
When introducing a new file into the project, please make sure it has a
copyright header making clear under which license it's being released.
 
# Developer Certificate of Origin (DCO)
 
To improve tracking of contributions to this project we will use a
process modeled on the modified DCO 1.1 and use a "sign-off" procedure
on patches that are being emailed around or contributed in any other
way.
 
The sign-off is a simple line at the end of the explanation for the
patch, which certifies that you wrote it or otherwise have the right
to pass it on as an open-source patch.  The rules are pretty simple:
if you can certify the below:
 
By making a contribution to this project, I certify that:
 
(a) The contribution was created in whole or in part by me and I have
    the right to submit it under the open source license indicated in
    the file; or
 
(b) The contribution is based upon previous work that, to the best of
    my knowledge, is covered under an appropriate open source License
    and I have the right under that license to submit that work with
    modifications, whether created in whole or in part by me, under
    the same open source license (unless I am permitted to submit
    under a different license), as indicated in the file; or
 
(c) The contribution was provided directly to me by some other person
    who certified (a), (b) or (c) and I have not modified it.
 
(d) The contribution is made free of any other party's intellectual
    property claims or rights.
 
(e) I understand and agree that this project and the contribution are
    public and that a record of the contribution (including all
    personal information I submit with it, including my sign-off) is
    maintained indefinitely and may be redistributed consistent with
    this project or the open source license(s) involved.
 
 
then you just add a line saying
 
    Signed-off-by: Random J Developer <random@example.org>
 
using your real name (sorry, no pseudonyms or anonymous contributions.)