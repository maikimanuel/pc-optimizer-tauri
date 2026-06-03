# 📖 PC Optimizer - Instrucciones de Compilación

## ✅ Opción 1: Script Python (RECOMENDADO)

### Requisitos:
- **Python 3.7+** desde https://www.python.org/downloads/
- ⚠️ **IMPORTANTE**: Durante la instalación, marca "Add Python to PATH"

### Pasos:

1. **Descarga e instala Python**
   - Ve a https://www.python.org/downloads/
   - Descarga Python 3.12 (o versión reciente)
   - Instala normalmente
   - **MARCA "Add Python to PATH"**

2. **Abre PowerShell como Administrador**
   - Presiona `Windows + X`
   - Selecciona "Windows PowerShell (Admin)"

3. **Navega a la carpeta del proyecto**
   ```powershell
   cd C:\ruta\a\pc-optimizer-tauri
   ```

4. **Ejecuta el script**
   ```powershell
   python build.py
   ```

5. **Espera a que termine**
   - Primera compilación: **5-10 minutos** (descarga dependencias)
   - Compilaciones futuras: **2-3 minutos**

6. **Tu .exe está en:**
   ```
   src-tauri\target\release\pc_optimizer.exe
   ```

---

## ✅ Opción 2: Compilación Manual

### Requisitos:
- **Node.js 16+**: https://nodejs.org/
- **Rust**: https://rustup.rs/

### Pasos:

```powershell
cd pc-optimizer-tauri
npm install
npm run tauri build
```

Espera 5-10 minutos y tu .exe estará listo en `src-tauri\target\release\pc_optimizer.exe`

---

## 🐛 Solución de Problemas

### Error: "python no se reconoce"
- Reinstala Python desde https://www.python.org/
- **MARCA "Add Python to PATH"** durante la instalación
- Reinicia PowerShell

### Error: "node no se reconoce"
- Descarga Node.js desde https://nodejs.org/
- Instala normalmente
- Reinicia PowerShell

### Error: "cargo no se reconoce"
- Descarga Rust desde https://rustup.rs/
- Reinicia PowerShell

### La compilación es muy lenta
- Normal en primera compilación (10-15 minutos)
- Compilaciones futuras: 2-3 minutos

---

**¡Disfruta PC Optimizer! 🎉**
