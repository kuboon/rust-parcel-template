import module from '../crate/Cargo.toml'
module.init();
const canvas = document.getElementById("canvas")
const ctx = canvas.getContext("2d")
let count = 0;
setInterval(()=>{
  module.draw(ctx)
  // count++;
  // const iData = ctx.createImageData(500,500)
  // const data = iData.data
  // for(let i=0;i<2000;i++)data[count*2000+i] = (i % 4)==1 ? 0 : 255
  // ctx.putImageData(iData, 0,0);
}, 0);
