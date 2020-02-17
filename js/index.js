import module from '../crate/Cargo.toml'
module.init();
const canvas = document.getElementById("canvas")
const ctx = canvas.getContext("2d")
setInterval(()=>{module.draw(ctx)}, 0);
