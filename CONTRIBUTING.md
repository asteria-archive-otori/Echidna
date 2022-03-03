# Echidna Contributing Guidelines
*Written by Nefo Fortressia. Markdown documents like this tend to not let you know who's the original author, huh?*

- [Commit Convention](#commit-convention)
  - [Commit Types](#commit-types)
  - [Commit Scopes](#commit-scopes)
- [PGP Signing](#pgp-signing)
- [Developer Certificate of Origin](#developer-certificate-of-origin)
- [Credits](#credits)

# Before you start coding....
In order to add new codes to the Echidna codebase, you'll need to have the dependencies installed. These are already written in the [Build from Source](./README#building-from-source).
 
After cloning the repository, be sure to install the Git hooks included.
```sh
$ pip install -U commitizen pre-commit
$ pre-commit install 
```

# Commit Convention
Let's not talk why we adopted [commit conventions]((https://www.conventionalcommits.org/en/)) for Echidna. The thing is, improper commit messages are just, GARRBAAAAGEEE. Messages like "what the fun does this commit do" or "ok ok ok ok" doesn't help at all in the log term.

These conventions are enforced with the use of [Commitizen](https://commitizen-tools.github.io/commitizen/).


#### Commit Types
We use the standard [Commitlint](https://github.com/conventional-changelog/commitlint/tree/master/%40commitlint/config-conventional#type-enum) commit types. :)

#### Commit scopes
At the moment, we don't use any commit scopes yet for Echidna, mainly because that hasn't been thought, hehe.

# PGP Signing
To ensure that your commits are really from *you*, you're required to sign your commits with PGP signatures. If you're new to PGP, GitHub made [a tutorial on making a PGP key with GnuPG](https://docs.github.com/en/authentication/managing-commit-signature-verification/generating-a-new-gpg-key) and [one on how to use it in your commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits), so do the instructions there and add the PGP public key to your Codeberg account, senpai!

# Licensing
By sending your code for addition in Echidna Code, you agree with the [Developer Certificate of Origin](./DCO.txt) and the [Mozilla Public License version 2](./LICENSE) licensing.

No *pwobwemwatwick* Contributor License Agreement, yay!

Shouwwwd twhe liwwcenswe fiwwles *nowt avwailwabwe*, ywou cwan gwet thwem awt https://developercertificate.org/ awnd https://mozilla.org/MPL/2.0/.

Pro tip, you can add `Signed-off-by: Takiyo Takahashi <takiyo@weeb.moe>` to the end line of your commit to officially agree to this by using the `-s` flag when making commits: 
```sh
$ git commit -s
```

This can also be enabled in VSCode as well. Just go to the settings and enable "Git: Always Sign Off".

# Credits
I have ~~stolen~~ borrowed KawalCOVID19's [Contributing Guides](https://github.com/kawalcovid19/wargabantuwarga.com/blob/main/CONTRIBUTING.md) as the basis for writing this file. I have significantly adjusted the language to my non-rant tone <3
