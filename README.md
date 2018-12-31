# Crom
Making version management easy

## What
Crom is a CLI that manages version numbers for you. You define a version format, Crom will take it from there.

Crom uses git tags to find the version, this means in CI, you don't need a way to update code.

For organizations that require code reviews for all changes, Crom updates versions only by tags.

## How
In `.crom.toml` you define a version format like `pattern = 'v0.1.%d'`. This tells from all versions should be `v0.1.#`. The `#` will be an incrementing number. Tags will include the whole prefix.

Crom makes it easy to integrate into any existing tool, as it will update other tools version definitions.

Crom currently supports:
 - `pom.xml` - Maven
 - `Cargo.toml` - Rust
 - `version.properties` - Anything that can use version.properties

# Artifacts
Crom is also able to upload built artifacts into GitHub. Making it easy to release artifacts to the rest of the world.