<div align="center">

# Uni 📦

A universal package manager wrapper to manage packages without difficulties

</div>

## Support 

<strong>
  
1. [NPM](https://www.npmjs.com/)
2. [YARN](https://classic.yarnpkg.com/en/docs/install#windows-stable)
3. [PNPM](https://pnpm.io/installation)
  
</strong>

> Many more to come

  
## Features
- Uses Rust under the hood for maximum performance
- Memory safe
- Lightweight
- Crossplatform
- Can use the usual syntax

## Usage

### Initialize with package.json

```
uni init
```

### Install packages

```
uni <install, i, add> <package-names>
```

>  uni will automatically prompt for the required package if there is no lock file

### Remove package

```
uni <remove, rm> <package-name>
```

### Create a sample project

```
uni create astro
```

### Default Wrapper commands

If lockfile is present
```
uni <instructions>
```

> Else, it will prompt for the required package name

## Future

- Extending it into a full wrapper for all the pakcage managers such as pip, cargo, flutter, etc...
- Code generation / Custom project generation

## LICENSE

### [MIT](https://github.com/Jenin-Immanuel/uni/blob/main/LICENSE)
