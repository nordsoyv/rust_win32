const js = import("./wasm/wasm_lib.js");

/*


  pub up_key: bool,
    pub down_key: bool,
    pub left_key: bool,
    pub right_key: bool,
    pub shoot_right: bool,
    pub shoot_left: bool,
    pub shoot_up: bool,
    pub shoot_down: bool,
    pub quit_key: bool,
    pub space: bool,

*/
js.then(js => {
  // js.greet("World!");
  js.init();
  let input = {
    up_key: true,
    left_key : false,
    right_key : false,
    down_key : false,
    shoot_down : false,
    shoot_up : false,
    shoot_left : false,
    shoot_right : false,
    quit_key : false,
    space : false,
  }
  let a = js.update(JSON.stringify(input), 0.0, 0.1 );
  let render = JSON.parse(a);
  console.log(render);

  let canvas = document.getElementById('canvas');
  let ctx = canvas.getContext('2d');

  render.forEach(r => {
    ctx.fillStyle = 'rgb(' + Math.floor(r.red) + ','+ Math.floor(r.green) + ','+ Math.floor(r.blue) + ')';
    ctx.fillRect(r.left, r.top, r.right - r.left, r.bottom- r.top);
  })

  // ctx.fillStyle = 'rgb(' + Math.floor(render[0].red) + ','+ Math.floor(render[0].green) + ','+ Math.floor(render[0].blue) + ')'
  // ctx.fillRect(render[0].)
});

