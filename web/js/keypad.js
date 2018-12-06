export const setupKeypad = (getCurrentKeyMappings, onKeyDown, onKeyRelease) => {
  window.addEventListener('keydown', (e) => {
    const keyMapping = getKeyMapping(e.keyCode);
    if (keyMapping) {
      onKeyDown(keyMapping.emuKey)
    }
  });

  window.addEventListener('keyup', (e) => {
    const keyMapping = getKeyMapping(e.keyCode);
    if (keyMapping) {
      onKeyRelease(keyMapping.emuKey);
    }
  });
  const getKeyMapping = (keyCode) => {
    return getCurrentKeyMappings().find(mapping => mapping.keyCode === keyCode);
  }
}



