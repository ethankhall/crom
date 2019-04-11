# Crom
Making version management easy

## What
Crom is a CLI that manages version numbers for you. You define a version format, Crom will take it from there.

Crom uses git tags to find the version, this means in CI, you don't need a way to update code.

For organizations that require code reviews for all changes, Crom updates versions only by tags.

## How
In `.crom.toml` you define a version format like `pattern = 'v0.1.%d'`. This tells from all versions should be `v0.1.#`. The `#` will be an incrementing number. Tags will include the whole prefix.

### Example

Image you have a few released version like below.

```
* 0dd81e7 - (tag: v0.1.4) Adding awesome feature #3
* dc03619 - (tag: v0.1.3) Adding awesome feature #2
* 557f909 - (tag: v0.1.2) Adding awesome feature #1
```

When you run `crom get current-version` and there are no working changes, you see `v0.1.4`.

You start working on awesome feature #4 and yor run `crom get current-version` when your git history looks like:

```
* 0cc81e3 - Adding awesome feature #4
* 0dd81e7 - (tag: v0.1.4) Adding awesome feature #3
* dc03619 - (tag: v0.1.3) Adding awesome feature #2
* 557f909 - (tag: v0.1.2) Adding awesome feature #1
```

Crom will tell you the version is `v0.1.5-SNAPSHOT` since there are local changes, and `v0.1.5` hasn't been released yet.

Now lets push to the repo. CI kicks off, instead of having to update a config file with every version you run `crom update-version --pre-release release`. This will update your version meta-data to be `v0.1.5`. 

Now you run your build. As the build executes and your code isn't to blame, but you need to fix something.

After fixing the change, you re-run `crom get current-version` and still see `v0.1.5-SNAPSHOT` since you don't release if a version doesn't build.

When you push, the history looks like:

```
* 9ad65c9 - Fixes Devin's bug.
* 0cc81e3 - Adding awesome feature #4
* 0dd81e7 - (tag: v0.1.4) Adding awesome feature #3
* dc03619 - (tag: v0.1.3) Adding awesome feature #2
* 557f909 - (tag: v0.1.2) Adding awesome feature #1
```

The CI job runs again, running `crom update-version --pre-release release` like before, and getting `v0.1.5` like before. This time however, the build passes.

Since the job passed, you want to tag it with `crom tag-version --source local,github --ignore-changes`. This creates a tag locally, and on GitHub. The tag locally is so you can use `crom upload-artifacts` without specifying a version.

Now on your local machine you run `git fetch && git pull` and see the history now looks like:

```
* 9ad65c9 - (tag: v0.1.5) Fixes Devin's bug.
* 0cc81e3 - Adding awesome feature #4
* 0dd81e7 - (tag: v0.1.4) Adding awesome feature #3
* dc03619 - (tag: v0.1.3) Adding awesome feature #2
* 557f909 - (tag: v0.1.2) Adding awesome feature #1
```

Running `crom get current-version` shows `v0.1.5`.

## Config Options
Here is an example `.crom.toml`.

```
pattern = 'v0.1.%d'
message-template = "Created {version} for release."

[cargo]
[maven]
[node]
[python]
path = "path/to/version.py"
[property]
path = "path/to/property-file.properties"
```


|         Name         |                                Description                                 |
| :------------------: | :------------------------------------------------------------------------: |
| `pattern` (required) |                 User defined format versions should take.                  |
|  `message-template`  |            When generating a `git tag` what should the text be?            |
|       `cargo`+       |             Specify that the crom should update Cargo configs              |
|       `maven`+       |           Specify that the crom should update Maven `pom.xml`'s.           |
|       `node`+        |         Specify that the crom should update node's `package.json`.         |
|      `python`+       | Specify that the crom should the specified file in a `version.py` format.  |
|     `property`+      | Specify that the crom should the specified file in a property file format. |

At least 1 of items marked with `+` need to also be included. 

### Pattern

The `pattern` field is completely completely user defined but is required to have a `%d`. The `%d` tells `crom` where you want the version to increment. In the example above, `crom` will create version `v0.1.0`, `v0.1.1`, `v0.1.2`, and so on. If you were to want a version more like an atomic incrementing number, you could use `%d` as the `pattern`.

In the event of a "hotfix" where a new part needs to be added to the version, you would just update the `pattern` to reflect that. In this example we would update pattern to be `v0.1.4.%d` if we needed to hotfix `v0.1.4`.

## Artifacts
Crom is also able to upload built artifacts into GitHub. Making it easy to release artifacts to the rest of the world.

Artifacts are defined in the config file `.crom.toml` and look like

```
[artifact.linux]
paths = { "crom" = "artifacts/linux/crom" }
target = "GitHub"
```

This will tell `crom` to upload a file at `artifacts/linux/crom` into Github, with the name `crom` for the current version.

The `paths` field can have multiple artifacts list, there can also be multiple named artifact "containers" to upload. In this case we have named it `linux` but it could be named anything.

### Compression
Some artifacts should be compressed before upload. To help with this, `crom` allows you to add a `compress` field into an artifact. An example of a compressed artifact is

```
[artifact.linux]
paths = { "crom" = "artifacts/linux/crom" }
target = "GitHub"
compress = { name = "linux.tar.gz", format = "tgz" }
```

The `name` field is the name of the artifact that will be uploaded, and `format` is which compression algorithm needs to be used. The `paths` field is used to determine which artifacts need to end up in the compressed file, and where they come from.

The `name` field should include the extension of the artifact.

| Supported Format |          Description           |
| :--------------: | :----------------------------: |
| `tgz`, `tar.gz`  | Build a tar.gz file to upload. |
|      `zip`       |  Build a zip file to upload.   |
