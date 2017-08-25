var connect = require('connect'),
    serveStatic = require('serve-static');

var app = connect();

app.use(serveStatic(__dirname));
app.listen(3334);

console.log('open localhost:3334');