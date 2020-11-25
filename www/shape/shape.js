import { TriangleRenderer } from "rusty_gl";

const renderer = TriangleRenderer.new("canvas", 200, 200);

renderer.render();

let update = function(e) {
  renderer.render();
}

document.getElementById("shape-red").addEventListener("change", update);
document.getElementById("shape-green").addEventListener("change", update);
document.getElementById("shape-blue").addEventListener("change", update);
document.getElementById("background-red").addEventListener("change", update);
document.getElementById("background-green").addEventListener("change", update);
document.getElementById("background-blue").addEventListener("change", update);
document.getElementById("vertex-count").addEventListener("change", update);
