import './style.css'
import * as osmos from './osmos-wasm'

const width = 1000
const height = 1000
const canvasElement = document.getElementById('canvas')

canvasElement.width = width
canvasElement.height = height
const ctx = canvasElement.getContext('2d')
const sim = new osmos.Simulator()
const render = () => {
  ctx.clearRect(0, 0, width, height)
  const objectList = sim.getObjectList()
  for (let object of objectList) {
    ctx.beginPath()
    ctx.fillStyle = '#8B008B'
    ctx.arc(object.x * width, object.y * height, object.energy, 0, 2 * Math.PI)
    ctx.fill()
  }
  sim.step()
  requestAnimationFrame(render)
}

render()