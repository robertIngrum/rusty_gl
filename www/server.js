const express = require('express');
const webpack = require('webpack');
const webpackDevMiddleware = require('webpack-dev-middleware');
const path = require('path');

const app = express();
const config = require('./webpack.config.js');
const compiler = webpack(config);

// Tell express to use the webpack-dev-middleware and use the webpack.config.js
// configuration file as a base.
app.use(
  webpackDevMiddleware(compiler, {
    publicPath: config.output.publicPath,
  })
);

app.get('/', (_req, res) => {
  res.sendFile(path.join(__dirname + '/index.html'));
});

app.get('/shape', (_req, res) => {
  res.sendFile(path.join(__dirname + '/shape/index.html'));
});

app.get('/shape/index.js', (_req, res) => {
  res.sendFile(path.join(__dirname + '/shape/index.js'));
});

app.get('/shape/shape.js', (_req, res) => {
  res.sendFile(path.join(__dirname + '/shape/shape.js'));
});

// Serve the files on port 3000.
app.listen(3000);
