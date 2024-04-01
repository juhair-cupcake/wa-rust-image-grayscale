(async () => {
  let rustApp = null;

  try {
    rustApp = await import("../pkg");
    console.log(rustApp);
  } catch (e) {
    console.error(e);
    return;
  }

  const input = document.getElementById("upload");
  const fileReader = new FileReader();

  fileReader.onloadend = () => {
    const base64 = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ""
    );

    document
      .getElementById("new-img")
      .setAttribute("src", rustApp.grayscale(base64));
  };

  input.addEventListener("change", () => {
    fileReader.readAsDataURL(input.files[0]);
  });
})();
