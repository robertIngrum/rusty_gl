import { TriangleRenderer } from "rusty_gl";

TriangleRenderer.new().render();

let update = function(e) {
  TriangleRenderer.new().render();
}

document.getElementById("shape-red").addEventListener("change", update);
document.getElementById("shape-green").addEventListener("change", update);
document.getElementById("shape-blue").addEventListener("change", update);
document.getElementById("background-red").addEventListener("change", update);
document.getElementById("background-green").addEventListener("change", update);
document.getElementById("background-blue").addEventListener("change", update);
