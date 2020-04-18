var connect = require('connect'),
    serveStatic = require('serve-static');
    path = require('path');
    fs = require('fs');

var app = connect();

app.use(serveStatic(__dirname));
app.use('/roms', function(req, res, next){
    if( req.method === 'GET' ){
        fs.readdir(path.join(__dirname, 'roms'), 'utf-8' , function(err, files){
            if( err ){
                console.log(err);
            } else{
                let roms = files.filter( file => /.+?\.nes$/.test(file) );
                res.end(JSON.stringify(roms));
            }
            next();
        });
    }
});
app.listen(3334);

console.log('open http://localhost:3334/');