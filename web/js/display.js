export const createDisplay = () => {
  const multiplier = 8;
  const canvas = document.querySelector("canvas");
  const canvasContext = canvas.getContext("2d");


  const setupDisplay = () => {
    canvas.width = 64 * multiplier;
    canvas.height = 32 * multiplier;
  }

  const drawPixels = (pixels) => {
    for (let row = 0; row < 32; row++) {
      const startOfRow = row * 64;
      const rowNumbers = pixels.slice(startOfRow, startOfRow + 64);

      for (let column = 0; column < 64; column++) {
        const pixel = rowNumbers[column];
        if (pixel) {
          canvasContext.fillStyle = "#26cdff";
          canvasContext.fillRect(column * multiplier, row * multiplier, multiplier, multiplier);
        } else {
          canvasContext.clearRect(column * multiplier, row * multiplier, multiplier, multiplier);
        }
      }
    }
  }

  setupDisplay();

  return {
    drawPixels
  }
}
