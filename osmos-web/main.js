import './style.css'
import * as osmos from './osmos-wasm'

const testBtn = document.getElementById('test')
testBtn.onclick =()=>{
  alert(osmos.hello())
}


