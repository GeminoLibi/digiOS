# Installing Ollama for digiOS

digiOS can use Ollama models for AI code generation. You have two options:

## Option 1: Ollama Server (Recommended)

The Ollama server provides the best performance and is the standard way to run models.

### Windows Installation

1. **Download Ollama:**
   - Visit: https://ollama.com/download
   - Download the Windows installer
   - Run the installer

2. **Verify Installation:**
   ```powershell
   ollama --version
   ```

3. **Start Ollama:**
   ```powershell
   ollama serve
   ```
   (This usually starts automatically on Windows)

4. **Pull a Model:**
   ```powershell
   ollama pull llama3.2
   # or
   ollama pull mistral
   # or
   ollama pull qwen2.5
   ```

5. **List Models:**
   ```powershell
   ollama list
   ```

## Option 2: Python Ollama Client (Fallback)

If you can't install Ollama server, digiOS can use the Python ollama client as a fallback.

### Setup

1. **Install Python ollama package:**
   ```powershell
   pip install ollama
   ```
   (You've already done this!)

2. **Make sure you have a model available:**
   - The Python client connects to Ollama server too
   - So you still need Ollama server running
   - OR use a different model provider

## Using with digiOS

1. **Start digiOS:**
   ```powershell
   .\target\release\aios.exe
   ```

2. **During setup, it will detect your Ollama models**

3. **Select an Ollama model** when prompted

4. **digiOS will use it for code generation**

## Troubleshooting

### "Ollama not found"
- Make sure Ollama is installed
- Check PATH: `where ollama`
- Restart terminal after installation

### "Ollama server not running"
- Start it: `ollama serve`
- Or it should auto-start on Windows

### "No models found"
- Pull a model: `ollama pull llama3.2`
- List models: `ollama list`

### Using Python Client
- digiOS will automatically fall back to Python client if server isn't available
- Make sure `python -c "import ollama"` works

## Recommended Models for Code Generation

- **llama3.2** - Good balance, fast
- **mistral** - High quality
- **qwen2.5** - Excellent for code
- **codellama** - Specialized for code (if available)

## Next Steps

Once Ollama is set up:
1. Run digiOS
2. It will detect your models
3. Select one during setup
4. Self-improvement will use it to generate real code!

