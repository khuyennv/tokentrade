import {
  initApp,
  transferSolToToken,
  transferTokenToSol,
} from "./networks";

async function main() {
  await initApp();
  switch (process.argv.slice(2)[0]) {
    case "1":
      await transferSolToToken();
      break;
    case "2":
      await transferTokenToSol();
      break;
    default:
      throw console.error("Invalid instruction");
  }

  console.log("Run commabnd success");
}

main().then(
  () => process.exit(),
  (err) => {
    console.error(err);
    process.exit(-1);
  }
);
