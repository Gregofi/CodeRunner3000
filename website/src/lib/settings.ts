export interface Settings {
  vimMode: boolean;
}

const defaultSettings: Settings = {
  vimMode: false,
};

export const getSettings = (): Settings => {
  const settings = localStorage.getItem("settings");
  if (settings) {
    return JSON.parse(settings);
  } else {
    return defaultSettings;
  }
};

export const setVimMode = (vimMode: boolean) => {
  const settings = getSettings();
  settings.vimMode = vimMode;
  setSettings(settings);
};

export const setSettings = (settings: Settings) => {
  localStorage.setItem("settings", JSON.stringify(settings));
};
