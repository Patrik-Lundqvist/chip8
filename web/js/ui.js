export const createUI = (onHalt, onStep, onNextGame, onPrevGame) => {
  const controlsDiv = document.querySelector('.controls');
  const debugDiv = document.querySelector('.debug-box');
  const debugInfoDiv = document.querySelector('.debug');
  const haltBtn = document.querySelector('.halt-button');
  const stepBtn = document.querySelector('.step-button');
  const debugBtn = document.querySelector('.debug-button');
  const nextGameBtn = document.querySelector('.next-game');
  const prevGameBtn = document.querySelector(".prev-game");
  const activeGameLabel = document.querySelector(".game-info-active");

  let debugInfoEntries = {};
  let showDebug = false;

  const setupEvents = () => {
    haltBtn.addEventListener("click", onHalt);
    stepBtn.addEventListener("click", onStep);
    debugBtn.addEventListener("click", toggleDebug);
    nextGameBtn.addEventListener("click", onNextGame);
    prevGameBtn.addEventListener("click", onPrevGame);
  }

  const setupDebugInfoEntries = () => {
    debugInfoEntries = {
      pc: addInfoBoxEntry(debugInfoDiv, 'PC'),
      i: addInfoBoxEntry(debugInfoDiv, 'I'),
    }
    for (let i = 0; i <= 15; i++) {
      debugInfoEntries[`v${i}`] = addInfoBoxEntry(debugInfoDiv, `V${i}`);
    }
  }

  const addInfoBoxEntry = (parentDiv, entryText) => {
    const controlEntry = document.createElement('div');
    controlEntry.classList.add('info-box-entry');
    controlEntry.innerText = entryText;
    parentDiv.append(controlEntry);
    return controlEntry;
  }

  const setupUI = () => {
    setupEvents();
    setupDebugInfoEntries();
    debugDiv.hidden = !showDebug;
  }

  const setGameLabel = (name) => {
    activeGameLabel.innerText = name;
  }

  const updateDebugInfo = (pc, i, v) => {
    if (!showDebug) {
      return;
    }
    debugInfoEntries.pc.innerText = `PC: ${pc}`;
    debugInfoEntries.i.innerText = `I: ${i}`;
    v.forEach((value, index) => {
      debugInfoEntries[`v${index}`].innerText = `V${index}: ${value}`;
    })
  }

  const updateControls = (keyMappings) => {
    while (controlsDiv.firstChild) {
      controlsDiv.firstChild.remove();
    }
    keyMappings.forEach((keyMapping) => addInfoBoxEntry(controlsDiv, keyMapping.description))
  }

  const toggleDebug = () => {
    showDebug = !showDebug;
    debugDiv.hidden = !showDebug;
  };

  setupUI();

  return {
    setGameLabel,
    updateDebugInfo,
    updateControls
  }
};
