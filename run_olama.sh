#!/bin/bash
#docker run -d -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama
delete_model() {
  echo "Deleting the model..."
  ollama rm matt-model
}

# Check for the --delete flag
if [[ "$1" == "--delete" ]]; then
  delete_model
  exit 0
fi

ollama create matt-model -f ./Modelfile
ollama serve matt-model 
