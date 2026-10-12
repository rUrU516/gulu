import { invoke } from "@tauri-apps/api/core";

type CalculatorView = {
  expression: string;
  display: string;
  error: string | null;
};

window.addEventListener("DOMContentLoaded", () => {
  const expressionEl = document.querySelector<HTMLElement>("#expression");
  const displayEl = document.querySelector<HTMLOutputElement>("#display");
  const errorEl = document.querySelector<HTMLElement>("#error");
  if (!expressionEl || !displayEl || !errorEl) return;

  function render(view: CalculatorView) {
    expressionEl!.textContent = view.expression;
    displayEl!.textContent = view.display;
    displayEl!.classList.toggle("long", view.display.length > 9);
    displayEl!.classList.toggle("very-long", view.display.length > 16);
    errorEl!.textContent = view.error ?? "";
    expressionEl!.scrollLeft = expressionEl!.scrollWidth;
    displayEl!.scrollLeft = displayEl!.scrollWidth;
  }

  // 按顺序发送按键，避免快速输入时把顺序打乱。
  let requests = Promise.resolve();
  function sendKey(key: string) {
    requests = requests.then(async () => {
      try {
        // 前端不拼接表达式、不计算，只把按键交给 Rust。
        const view = await invoke<CalculatorView>("calculator_key", { key });
        render(view);
      } catch (error) {
        errorEl!.textContent = typeof error === "string"
          ? error
          : "暂时无法计算，请重新打开应用";
        console.error(error);
      }
    });
  }

  document.querySelectorAll<HTMLButtonElement>("[data-key]").forEach((button) => {
    button.addEventListener("click", () => {
      const key = button.dataset.key;
      if (key) sendKey(key);
    });
  });

  document.addEventListener("keydown", (event) => {
    if (event.metaKey || event.ctrlKey || event.altKey) return;

    let key = event.key;
    if (key === "Enter" || key === "=") key = "equals";
    else if (key === "Escape") key = "clear";
    else if (key === "Backspace" || key === "Delete") key = "backspace";
    else if (!/^[0-9.+\-*/()]$/.test(key)) return;

    event.preventDefault();
    sendKey(key);
  });

  // 页面重载时，从 Rust 读取当前显示，不丢失正在输入的内容。
  sendKey("snapshot");
});
