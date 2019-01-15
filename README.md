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

## Artifacts
Crom is also able to upload built artifacts into GitHub. Making it easy to release artifacts to the rest of the world.