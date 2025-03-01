window.snowEnabled = false;
window.onload = () => {};
function createSnowEffect() {
  window.snowEnabled = true;
  const indexLayout = document.querySelector("body");
  function createSnowflake() {
    if (!window.snowEnabled) {
      return 0;
    }
    const snowflake = document.createElement("div");
    snowflake.textContent = "❄";
    Object.assign(snowflake.style, {
      position: "absolute",
      top: "-10px",
      color: "white",
      userSelect: "none",
      fontSize: `${Math.random() * 10 + 10}px`,
      left: `${Math.random() * 100}vw`,
      opacity: Math.random().toFixed(2),
      animation: `fall ${Math.random() * 5 + 5}s linear infinite`,
    });
    indexLayout.appendChild(snowflake);
    setTimeout(() => snowflake.remove(), (Math.random() * 5 + 5) * 1000);
  }
  const style = document.createElement("style");
  style.textContent = `
        @keyframes fall {
            0% {
                transform: translateY(-10px);
                opacity: 0.8;
            }
            100% {
                transform: translateY(110vh);
                opacity: 0;
            }
        }
    `;
  document.head.appendChild(style);
  setInterval(createSnowflake, 200);
}
