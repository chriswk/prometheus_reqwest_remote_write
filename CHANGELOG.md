# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.3.0 (2025-02-25)

### Bug Fixes

 - <csr-id-78513fb369ed7790afb6d85160eea61774e88371/> remove openssl dependency. Use rustls instead

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release prometheus-reqwest-remote-write v0.3.0 ([`e7c932f`](https://github.com/chriswk/prometheus_reqwest_remote_write/commit/e7c932f80f99891c015935a41b1b3d8019efcef2))
    - Remove openssl dependency. Use rustls instead ([`78513fb`](https://github.com/chriswk/prometheus_reqwest_remote_write/commit/78513fb369ed7790afb6d85160eea61774e88371))
</details>

## v0.2.1 (2024-10-16)

### Bug Fixes

 - <csr-id-3ae738c039c7d5a08dadacb0ee41be82b59a9294/> actually add custom labels for histograms, summaries and gauges

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release prometheus-reqwest-remote-write v0.2.1 ([`0657641`](https://github.com/chriswk/prometheus_reqwest_remote_write/commit/0657641e15530bfe79b76c387d01b32e32529732))
    - Actually add custom labels for histograms, summaries and gauges ([`3ae738c`](https://github.com/chriswk/prometheus_reqwest_remote_write/commit/3ae738c039c7d5a08dadacb0ee41be82b59a9294))
</details>

## v0.2.0 (2024-10-16)

<csr-id-c336bb9415378e478d2dc3c0dd3440cc2e971fe2/>

### Chore

 - <csr-id-c336bb9415378e478d2dc3c0dd3440cc2e971fe2/> add license

### New Features

 - <csr-id-c8e7683aa4bb802b7d43fc636622d8077840a25a/> add possibility to add labels added to all metrics in write request

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release prometheus-reqwest-remote-write v0.2.0 ([`b3ce210`](https://github.com/chriswk/prometheus_reqwest_remote_write/commit/b3ce2105d4de5ced9112deb288a8d17d658c9c9e))
    - Add possibility to add labels added to all metrics in write request ([`c8e7683`](https://github.com/chriswk/prometheus_reqwest_remote_write/commit/c8e7683aa4bb802b7d43fc636622d8077840a25a))
    - Add license ([`c336bb9`](https://github.com/chriswk/prometheus_reqwest_remote_write/commit/c336bb9415378e478d2dc3c0dd3440cc2e971fe2))
</details>

## v0.1.1 (2024-10-15)

<csr-id-8398e100be7d6fcb6ea582ea78c2d3491a70271e/>

### Chore

 - <csr-id-8398e100be7d6fcb6ea582ea78c2d3491a70271e/> remove prometheus-parse. We only support MetricFamilies

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release prometheus-reqwest-remote-write v0.1.1 ([`800b729`](https://github.com/chriswk/prometheus_reqwest_remote_write/commit/800b729e6ffac098bc8cf26096411114b6c1c874))
    - Remove prometheus-parse. We only support MetricFamilies ([`8398e10`](https://github.com/chriswk/prometheus_reqwest_remote_write/commit/8398e100be7d6fcb6ea582ea78c2d3491a70271e))
</details>

## v0.1.0 (2024-10-15)

<csr-id-48d7cb3d730f5fb83fe9b77c92dcd8924b22c4f7/>
<csr-id-b71e4bfac54f6e0fdcca3e48754a1723bfcf3743/>

### Chore

 - <csr-id-48d7cb3d730f5fb83fe9b77c92dcd8924b22c4f7/> add workflow for running build

### Chore

 - <csr-id-b71e4bfac54f6e0fdcca3e48754a1723bfcf3743/> add changelog

### New Features

 - <csr-id-a58402e5b6b31688581a1359939e21cc8d971aaa/> Can now aggregate Gauge, Counter and Histogram
   Initial commit to this repository. Converts MetricFamiliy samples into a
   WriteRequest for pushing to Prometheus's Remote Write protocol.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release prometheus-reqwest-remote-write v0.1.0 ([`36ad630`](https://github.com/chriswk/prometheus_reqwest_remote_write/commit/36ad6300a952c0f10978a15c0dd472dcde5d60e1))
    - Add changelog ([`b71e4bf`](https://github.com/chriswk/prometheus_reqwest_remote_write/commit/b71e4bfac54f6e0fdcca3e48754a1723bfcf3743))
    - Add workflow for running build ([`48d7cb3`](https://github.com/chriswk/prometheus_reqwest_remote_write/commit/48d7cb3d730f5fb83fe9b77c92dcd8924b22c4f7))
    - Can now aggregate Gauge, Counter and Histogram ([`a58402e`](https://github.com/chriswk/prometheus_reqwest_remote_write/commit/a58402e5b6b31688581a1359939e21cc8d971aaa))
</details>

