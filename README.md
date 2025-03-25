<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a id="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![project_license][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/MaikBuse/syndicode">
    <img src="images/logo.png" alt="Logo" width="160" height="160">
  </a>

<h3 align="center">Syndicode</h3>
<p align="center"><em>Compile your power. Execute your enemies.</em></p>

  <p align="center">
    A competitive strategy game for programmers set in a cyberpunk Japan —  
    where digital warfare meets ancient honor. Lead a covert syndicate through  
    the neon-lit underworld of Tokyo, manipulating code, loyalty, and power.
    <br />
    <a href="https://github.com/MaikBuse/syndicode"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/MaikBuse/syndicode">View Demo</a>
    &middot;
    <a href="https://github.com/MaikBuse/syndicode/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    &middot;
    <a href="https://github.com/MaikBuse/syndicode/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
  </p>
</div>


<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![Syndicode][product-screenshot]](https://github.com/MaikBuse/syndicode)

Syndicode is not a traditional game. It is a digital battleground — a strategy engine running as a gRPC server, built to be language-agnostic and fiercely open-ended.

You are not just a player. You are a programmer, strategist, and syndicate architect. The core server exposes a powerful API designed to be accessed from any language — Rust, Python, Go, TypeScript, C++, and beyond.

There is no official client. You are expected to build your own, tailor it to your style, optimize it for control, and extend it to dominate.

Will you craft a sleek tactical dashboard? A code-driven command-line brain? Or an AI that plays while you sleep?

Your creativity is your weapon. Your code is your empire.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


### Built With

* [![Rust][Rust]][rust-url]
* [![SQLite][SQLite]][sqlite-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## 🛠️ Getting Started: Build Your Own Client

*Syndicode doesn’t give you a UI — it gives you a protocol.*
To play, you’ll write your own client using the provided gRPC API. Your client can be a simple script, a terminal-based dashboard, or a fully-automated AI — the only limit is your creativity.

### Step 1: Pick a Language

Syndicode is powered by [gRPC](https://grpc.io/), with support for:

- Rust  
- Python  
- Go  
- C++  
- JavaScript / TypeScript  
- Java  
- …and any other gRPC-compatible language

> 💡 **Pro tip**: Choose a language you’re comfortable in — or one you want to weaponize.



### Step 2: Connect to the server

Connect to the official public server at `https://api.syndicode.dev`.



### Step 3: Get the API Definition

You can either:

Use [gRPC server reflection](https://grpc.io/docs/guides/reflection/) with the official public server.

Or clone the repo or pull the `.proto` files:

```bash
git clone https://github.com/MaikBuse/syndicode.git
cd syndicode/server/proto
```



### Step 4: Build, Adapt, Compete
You now have full access to your syndicate’s digital brain.
What you build is up to you:

* A tactical terminal UI
* A real-time strategy GUI
* A self-learning bot
* A turn-based decision tree
* Or something no one's thought of yet



## Self-host the server

In case you would rather self-host your own server, here are the intructions.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.
* sqlx
  ```sh
  cargo install sqlx-cli
  ```
* just
  ```sh
  cargo install just
  ```

### Installation

1. Clone the repo
   ```sh
   git clone git@github.com:MaikBuse/syndicode.git
   ```


2. Setup environment variables

| Name           | Description                                                                                 | Optional | Example                    |
|----------------|---------------------------------------------------------------------------------------------|----------|----------------------------|
| DATABASE_URL   | The url of the sqlite database                                                              | False    | "sqlite:syndicode.db"      |
| JWT_SECRET     | The secret used to encrypt the json web token                                               | False    | "some-super-secret-string" |
| ADMIN_PASSWORD | The password of the default user created on application startup. The user's name is 'admin' | False    | "my-great-password"        |


3. Run the server
   ```sh
   just server-run
   ```


4. Change git remote url to avoid accidental pushes to base project when making changes yourself
   ```sh
   git remote set-url origin github_username/repo_name
   git remote -v # confirm the changes
   ```


<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Gameplay

Use this space to show useful examples of how a project can be used. Additional screenshots, code examples and demos work well in this space. You may also link to more resources.

_For more examples, please refer to the [Documentation](https://example.com)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [ ] Feature 1
- [ ] Feature 2
- [ ] Feature 3
    - [ ] Nested Feature

See the [open issues](https://github.com/MaikBuse/syndicode/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Top contributors:

<a href="https://github.com/MaikBuse/syndicode/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=MaikBuse/syndicode" alt="contrib.rocks image" />
</a>



<!-- LICENSE -->
## License

Distributed under the project_license. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Maik Buse - [Website](https://maikbuse.com) - contact@maikbuse.com

Project Link: [https://github.com/MaikBuse/syndicode](https://github.com/MaikBuse/syndicode)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* []()
* []()
* []()

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/MaikBuse/syndicode.svg?style=for-the-badge
[contributors-url]: https://github.com/MaikBuse/syndicode/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/MaikBuse/syndicode.svg?style=for-the-badge
[forks-url]: https://github.com/MaikBuse/syndicode/network/members
[stars-shield]: https://img.shields.io/github/stars/MaikBuse/syndicode.svg?style=for-the-badge
[stars-url]: https://github.com/MaikBuse/syndicode/stargazers
[issues-shield]: https://img.shields.io/github/issues/MaikBuse/syndicode.svg?style=for-the-badge
[issues-url]: https://github.com/MaikBuse/syndicode/issues
[license-shield]: https://img.shields.io/github/license/github_username/repo_name.svg?style=for-the-badge
[license-url]: https://github.com/MaikBuse/syndicode/blob/master/LICENSE.txt
[product-screenshot]: images/hero.png
[Rust]: https://img.shields.io/badge/Rust-000?logo=rust&logoColor=white&labelColor=000&style=flat-square
[rust-url]: https://rust-lang.org/
[SQLite]: https://img.shields.io/badge/SQLite-003B57?style=flat-square&logo=sqlite&logoColor=white
[sqlite-url]: https://sqlite.org/
