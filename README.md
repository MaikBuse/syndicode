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
    A competitive strategy game for programmers set in cyberpunk Japan —  
    where digital warfare meets ancient honor. Lead a covert syndicate through  
    neon-lit Tokyo, manipulating code, loyalty, and power.
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


<!-- ABOUT THE PROJECT -->
## About The Project

[![Syndicode][product-screenshot]](https://github.com/MaikBuse/syndicode)

Syndicode isn't a traditional game — it's a digital battleground.

Built as a gRPC server, it provides an open-ended, language-agnostic playground for strategy, automation, and digital subterfuge.

There’s no official UI. You build your own client — script, dashboard, or bot — and use it to dominate Tokyo’s neon-shadowed underworld.

*Your creativity is your weapon. Your code is your empire.*

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## 🎮 Gameplay

*Syndicode is a competitive strategy game where players control powerful syndicates in futuristic Tokyo. These factions operate publicly — managing assets, negotiating influence, and executing code-driven decisions across every layer of the city.*

---

### 💴 Economy

<img src="images/gameplay/economy.png" alt="Economy gameplay" width="100%" />

*Syndicates influence the city’s markets through trade, logistics, and digital currency. From corporate front operations to underground exchanges, economic power is built in broad daylight.*

---

### 🕶️ Espionage

<img src="images/gameplay/espionage.png" alt="Espionage gameplay" width="100%" />

*Information is currency. Whether gathering intel, intercepting transmissions, or manipulating data flows — espionage operations are quiet, but their consequences are public.*

---

### ⚔️ Warfare

<img src="images/gameplay/warfare.png" alt="Warfare gameplay" width="100%" />

*When diplomacy fails, syndicates strike. Automated units, drones, and digital warfare tools are deployed to protect territory, settle disputes, or send a message — sometimes in full view of the city.*

---

### 🕊️ Diplomacy

<img src="images/gameplay/diplomacy.png" alt="Diplomacy gameplay" width="100%" />

*Broker alliances with politicians, factions, and rival syndicates in ritualized negotiations under glowing sakura and digital crests. Every agreement is a power move — and every peace, temporary.*

---

> *No single path leads to dominance, but specialization might get you ahead.*


<p align="right">(<a href="#readme-top">back to top</a>)</p>

<br>

<!-- GETTING STARTED -->
## 🛠️ Getting Started: Build Your Own Client

*Syndicode doesn’t give you a UI — it gives you a protocol.*

To play, you’ll write your own client using the provided gRPC API. Your client can be anything:

- A CLI script
- A tactical dashboard
- A fully autonomous AI

The only limit is your creativity.

---

### Step 1: Pick a Language

Syndicode uses [gRPC](https://grpc.io/), with support for:

* Rust  
* C# / .NET
* C++
* Dart
* Go
* Java
* Kotlin
* Node
* Objective-C
* PHP
* Python
* Ruby
* …and any other gRPC-compatible language

> 💡 **Pro tip**: Choose a language you already know — or maybe one you want to learn!

<br>

### Step 2: Connect to the server

Use the official public server:

```txt
https://api.syndicode.dev
```

<br>

### Step 3: Get the API Definition

Use gRPC server reflection *or* clone the `.proto` files:

```bash
git clone https://github.com/MaikBuse/syndicode.git
cd syndicode/server/proto
```

<br>

### Step 4: Build, Adapt, Compete

You now control your syndicate’s digital brain. Build anything:

* A terminal UI
* A real-time GUI
* A self-learning AI
* A turn-based engine
* Or something no one’s imagined yet

<br><br>

## ☁️ Self-Host the Server

Prefer to run your own server? Here's how.

### Built with
* [![Rust][Rust]][rust-url]
* [![PostgreSQL][PostgreSQL]][postgresql-url]

---

### Prerequisites

Install these tools first:

#### just
A handy command runner that simplifies development workflows with reusable recipes — think of it like a more powerful Makefile for Rust projects.
```bash
cargo install just
```
You can print out all available recipes with the command `just` while being in the project root directory.

#### sqlx-cli
Enables you to run database migrations and prepare SQLx query metadata for compile-time validation.
```bash
cargo install sqlx-cli
```

---

### Installation

1. Clone the repo:
   ```bash
   git clone git@github.com:MaikBuse/syndicode.git
   ```

2. Copy the dot env example from `./examples/.env.example` and paste it under the project's root directory as `./.env`. Both just and sqlx-cli will now use the provided environment variables.

| Name            | Description                                                                                 | Required | Example                    |
|-----------------|---------------------------------------------------------------------------------------------|----------|----------------------------|
| `DATABASE_URL`  | SQLite database path                                                                        | ✅       | `sqlite:syndicode.db`      |
| `JWT_SECRET`    | Secret key for token signing                                                                | ✅       | `some-super-secret-string` |
| `ADMIN_PASSWORD`| Password for the default admin user (`admin`)                                               | ✅       | `my-great-password`        |

3. Start the database
   ```bash
   just db start
   ```

4. Setup the database
   ```bash
   just db setup
   ```

5. Start the server:
   ```bash
   just server run
   ```

6. Reset Git remote (to avoid pushing to original):
   ```bash
   git remote set-url origin git@github.com:your-username/your-repo.git
   git remote -v  # verify changes
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<br>

## 🤝 Contributing

Contributions make the open-source world better — and this project stronger.

If you have an idea, open a feature request. If you find a bug, submit a report. If you want to get hands-on, fork the repo and build something awesome.

> ⭐ Don’t forget to star the project if you like it!

### Getting Started:

1. Fork the repo
2. Create your branch: `git checkout -b feature/AmazingFeature`
3. Commit your changes: `git commit -m 'Add AmazingFeature'`
4. Push the branch: `git push origin feature/AmazingFeature`
5. Open a Pull Request

See [open issues](https://github.com/MaikBuse/syndicode/issues) for current ideas.

### Top Contributors:

<a href="https://github.com/MaikBuse/syndicode/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=MaikBuse/syndicode" />
</a>

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<br>

## 📚 License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<br>

## 💬 Contact

Maik Buse - [Website](https://maikbuse.com) - contact@maikbuse.com

Project Link: [https://github.com/MaikBuse/syndicode](https://github.com/MaikBuse/syndicode)

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
[contributors-shield]: https://img.shields.io/github/contributors/MaikBuse/syndicode.svg?style=for-the-badge
[contributors-url]: https://github.com/MaikBuse/syndicode/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/MaikBuse/syndicode.svg?style=for-the-badge
[forks-url]: https://github.com/MaikBuse/syndicode/network/members
[stars-shield]: https://img.shields.io/github/stars/MaikBuse/syndicode.svg?style=for-the-badge
[stars-url]: https://github.com/MaikBuse/syndicode/stargazers
[issues-shield]: https://img.shields.io/github/issues/MaikBuse/syndicode.svg?style=for-the-badge
[issues-url]: https://github.com/MaikBuse/syndicode/issues
[license-shield]: https://img.shields.io/github/license/MaikBuse/syndicode.svg?style=for-the-badge
[license-url]: https://github.com/MaikBuse/syndicode/blob/master/LICENSE.txt
[product-screenshot]: images/hero.png
[rust]: https://img.shields.io/badge/Rust-4A4A55?style=for-the-badge&logo=rust&logoColor=FF3E00
[rust-url]: https://rust-lang.org/
[postgresql]: https://img.shields.io/badge/PostgreSQL-4A4A55?style=for-the-badge&logo=postgresql&logoColor=FF3E00
[postgresql-url]: https://postgresql.org/
