export const random = () => {
    return Math.random();
}

export const log = s => {
    console.log(`FROM RUST: ${s}`);
}

let ctx = null;

export const start_frame = () => {
    let canvas = document.getElementById('canvas');
    ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, 960, 540);
}

export const end_frame = () => {
}

export const draw_rectangle = (min_x, min_y, max_x, max_y, red, green, blue) => {
    let width = max_x - min_x;
    let height = max_y - min_y;
    let left = min_x;
    let top = 540 - max_y;
    ctx.fillStyle = 'rgb(' + Math.floor(red * 255) + ',' + Math.floor(green * 255) + ',' + Math.floor(blue * 255) + ')';
    ctx.fillRect(left, top, width, height);

}
