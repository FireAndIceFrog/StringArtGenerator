import type { WorkerMessage } from "../workers/lineLengthWorker";

const worker = new Worker(
  new URL("../workers/lineLengthWorker.ts", import.meta.url),
  { type: "module" }
);

export async function generateLineLength(args: WorkerMessage): Promise<number> {
  if (!worker) {
    throw new Error("Service worker not initialized");
  }

  return new Promise((resolve, reject) => {
    worker.onmessage = (event) => {
      const { type, data } = event.data;
    
    if (type === "complete") {
        resolve(data);
    } else if (type === "error") {
        reject(new Error(data));
    }
    };

    worker.postMessage(args);
  });
}
