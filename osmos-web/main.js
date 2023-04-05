import './style.css'
import * as osmos from './osmos-wasm'


const width = 1000
const height = 1000
let canvasElement = document.getElementById("canvas");
canvasElement.width = width
canvasElement.height = height

const ctx = canvasElement.getContext("2d");
const sim = new osmos.Simulator();


const render = () => {
    ctx.clearRect(0, 0, width, height);
    console.log(sim)
    console.log(sim.get_object_list())
    const objectList = sim.get_object_list();
    console.log(123)
    console.log(objectList)
    for (let object of objectList) {
        ctx.beginPath();
        ctx.fillStyle = '#F47C7C';
        ctx.arc(object.x * width, object.y * height, object.energy, 0, 2 * Math.PI);
        ctx.fill();
    }
    requestAnimationFrame(render);
}
render()
