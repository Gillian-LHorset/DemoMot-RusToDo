# Mise en place du backend

## Mise en place de l'espace de travail

Installez les outils de développement C++ de Visual Studio (Développement Desktop en C++).

Installer Rust depuis le [site officiel](https://rustup.rs/).

## Créer le .env

Copiez le fichier `.env.exemple` et changer l'url pour correspondre à votre container docker.  
(par défaut : `DATABASE_URL=postgres://postgres_user:postgres_password@localhost:5432/rustodo-db`)

## Installation des dépendances

Installez les outils de développement C++ de Visual Studio
Il suffit de bulid le programme avec `rust build`

OU

Installer sous-système Debian  
Update Debian  
Installer les outils de développement C++ avec la commande : `sudo apt update && sudo apt install -y build-essential pkg-config libssl-dev`  
Installer rust dans ce système `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` (install standard)  
Dans RustRover : Ajouter une configuration pour la compilation qui run avec `WSL - Debian`  
Executez les commandes `which rustc` et `which cargo`  
Ajouter les executables rust et cargo dans les targets du profil RustRover

## Lancer la base de données

Ouvrez un terminal Debian dans l'environnement puis installer sqlx-cli avec cette commande :
`cargo install sqlx-cli --no-default-features --features postgres`

Commande pour créer les migrations :

```sh
sqlx database reset
```
