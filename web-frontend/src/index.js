const js = import("./wasm/wasm_lib.js");

let input = {
    up_key: false,
    left_key: false,
    right_key: false,
    down_key: false,
    shoot_down: false,
    shoot_up: false,
    shoot_left: false,
    shoot_right: false,
    quit_key: false,
    space: false,
};

let update;
let startTime = 0;
let lastFrameTime = 0;


js.then(js => {
    update = js.update;
    document.addEventListener('keydown', event => {

        if (event.key === 'w') input.up_key = true;
        if (event.key === 'a') input.left_key = true;
        if (event.key === 's') input.down_key = true;
        if (event.key === 'd') input.right_key = true;
        if (event.key === 'i') input.shoot_up = true;
        if (event.key === 'j') input.shoot_left = true;
        if (event.key === 'k') input.shoot_down = true;
        if (event.key === 'l') input.shoot_right = true;
        if (event.key === 'q') input.quit_key = true;
        if (event.key === ' ') input.space = true;
        //  console.log(input);
    })

    document.addEventListener('keyup', event => {
        if (event.key === 'w') input.up_key = false;
        if (event.key === 'a') input.left_key = false;
        if (event.key === 's') input.down_key = false;
        if (event.key === 'd') input.right_key = false;
        if (event.key === 'i') input.shoot_up = false;
        if (event.key === 'j') input.shoot_left = false;
        if (event.key === 'l') input.shoot_right = false;
        if (event.key === 'k') input.shoot_down = false;
        if (event.key === 'q') input.quit_key = false;
        if (event.key === ' ') input.space = false;
    })

    js.init();
    startTime = performance.now();
    requestAnimationFrame(mainLoop);
});

let frameCounter = 0;

const mainLoop = () => {
    let currentTime = performance.now();
    let elapsedTime = (currentTime - startTime) / 1000;
    let delta = (currentTime - lastFrameTime) /1000;
    lastFrameTime = currentTime;
    update(JSON.stringify(input), elapsedTime, delta);
    requestAnimationFrame(mainLoop);
    frameCounter++;
};
