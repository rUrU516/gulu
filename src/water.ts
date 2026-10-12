import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

const status = document.querySelector<HTMLElement>("#status");
const remindButton = document.querySelector<HTMLButtonElement>("#remind");
const dismissButton = document.querySelector<HTMLButtonElement>("#dismiss");

remindButton?.addEventListener("click", async () => {
  remindButton.disabled = true;
  try {
    // Rust 负责打开真正的提醒窗口。
    await invoke("remind_now");
    if (status) status.textContent = "喝水提醒已打开。";
  } catch (error) {
    if (status) status.textContent = String(error);
  } finally {
    remindButton.disabled = false;
  }
});

dismissButton?.addEventListener("click", async () => {
  try {
    await getCurrentWindow().close();
  } catch (error) {
    if (status) status.textContent = String(error);
  }
});
