import module from '../Cargo.toml'
module.init();
const canvas = document.getElementById("canvas")
const ctx = canvas.getContext("2d")
let count = 0;
setInterval(()=>{
  const ret = module.draw(ctx)
}, 1);
