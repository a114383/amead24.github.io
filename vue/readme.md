# Vue Mastery

Understanding the framework fundamentals seems easier than understanding the javascript ecosystem that one has to know before even getting started. Here's some notes/thoughts i've had so far:

`npm` - Node Package Manager, central repository for hosting javascript packages. After updating the permissions on your \$USER directory everything so far has suggested installing with `npm install --global <package>` which seems counter intuitive to having a package.json as the project metadata/dependencies files?

I'm also not sure why `npm` knows how to serve up the application on localhost versus `vue` doing that. The equivalent seems like using pip to serve flask? Regardless you can run:

```bash
npm run serve  # to start a web server
npm run build  # to create dist/
```

`vue cli` - Installed with: `npm install -g @vue/cli` which allows for you to create projects with set defaults. and any additional stuff, or defaults can be set inside `~/.vuerc`:

```bash
vue create hello-world  # initialize a new project
vue ui                  # create a project with a UI & install plugins

```

`webpack` - ???

`babel` - ???
