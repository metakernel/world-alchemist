# World Alchemist

> [!WARNING]
> This project is currently in early development, is not yet ready for production use and may be unusable at this stage. Features and functionalities are subject to change.

## Why This Project?

In my years of experience as a game developer and game director, i've often found that there were discrepencies between design and actual implementation. But more than that, as we thrive to create entertaining and compelling game experiences for players screaming for larger and livid worlds; design and development teams are often overwhelmed by the amount of content that needs to be created while studios are often limited in size, time and budget.

It is particularly a reality for indie studios and solo developers and while they are often the innovators of the industry, they are also the most limited in resources to create large scale games with rich and diverse worlds. This make it particularly difficult for them to innovate while working on narrative rich games with large worlds.

This is where the idea of **World Alchemist** was born. Creating a tool that would help developers document and build their worlds, implement gameplay systems and connect narratives all of that while actually beeing able to prototype, extract game design documents and generate engine agnostic contents.

## What is World Alchemist ?

World Alchemist is the most powerful Design-First gave development tool ever created, it is developped using the Rust programming language so that it offers performance and compatibility with the C ABI for eventual integration with game engines and other languages. It brings together worldbuilding, narrative design, game design and prototyping into your terminal through its `world` command line tool offering a powerful REPL scripting environment for the Alembscript DSL based on the MMS (for Model-Mechanic-Signal) framework so that you can transform your ideas into fully fledged game worlds.C

## Features

Here are some of the main features i am intending to implement first for **World Alchemist**:

- Discover Alembscript, a powerful scripting language based on the MMS (Model-Mechanic-Signal) framework to define your game mechanics, systems and interactions.
- Use the Alembscript REPL in Authoring, Prototype or Query modes to build every aspect of your world brick by brick, interact with it or query its data.
- Craft every aspect of your game world, from geography to cultures, histories, and ecosystems directly from the cli through interactive commands (Using Ratatui).
- Human and version control friendly dataformat (TOML) to store and manage your world data while keeping them versionned alongside your game project.
- Build every facets of your game using World Alchemist's `WMMS`, an implementation of what i called the `Model-Mechanic-Signal (MMS)` framework.
- Define the entities that populate your world, including characters, items, faction, cities, etc. through a powerful data-driven approach.
- Design intricate narratives with branching storylines, character arcs, and dialogues.
- Implement and test gameplay mechanics and systems.

Then for later down the road:

- Generate comprehensive game design documents from your world data and export them to various formats.
- Prototype your game ideas quickly and cost-effectively through a simple embedded game engine before committing to full development.
- Create a real world map representation with procedural generation tools, connected to your world data and visualized through an interactive map interface (2D and 3D) or export it.
- Export your world data in various formats or use them directly into various game engines, programming languages and platforms.
- A plugin system to extend World Alchemist's capabilities and integrate with other tools and services.
- A graphical user interface (GUI) for those who prefer visual interaction over command-line interfaces.
- Collaboration features to allow multiple designers and developers to work on the same world project simultaneously.
- Integration with popular game engines like Unity, Unreal Engine, Godot and Bevy.
