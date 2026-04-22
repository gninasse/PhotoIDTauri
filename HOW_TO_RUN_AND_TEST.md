# Instructions pour Exécuter et Tester Visuellement le Projet

Ce fichier documente les étapes pour lancer le projet et le tester visuellement, ainsi que les outils et patterns utilisés pendant le processus de développement.

## Comment Exécuter le Projet

1. **Installation des dépendances :**
   Assurez-vous d'avoir installé toutes les dépendances requises (par exemple, via `npm install`, `yarn install` ou `pip install` selon la nature du projet).

2. **Démarrage du serveur local :**
   Lancez le serveur de développement local :
   - Pour un projet Node.js : `npm start` ou `npm run dev`
   - Pour un projet Python : `python app.py` ou `flask run` (selon le framework)

   *Note:* Si le projet est un simple frontend statique, vous pouvez utiliser un serveur HTTP léger, par exemple : `python3 -m http.server 3000`.

## Comment Tester Visuellement

Pour vérifier visuellement l'application de manière automatisée, nous utilisons souvent des scripts de capture d'écran via des navigateurs headless comme Playwright.

### Utilisation de Playwright (Exemple)

```bash
# Initialiser un projet Node.js si ce n'est pas fait
npm init -y
# Installer Playwright
npm install playwright
```

Créer un script simple (`verify.js`) pour prendre une capture d'écran :

```javascript
const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage();

  // Remplacer par l'URL locale de votre application
  await page.goto('http://localhost:3000');

  // Prendre une capture d'écran
  await page.screenshot({ path: 'screenshot.png' });

  await browser.close();
})();
```

Exécutez ensuite le script avec Node.js :
```bash
node verify.js
```
La capture sera disponible sous `screenshot.png` pour une inspection visuelle.

## Patterns et Outils Utilisés

Au cours du développement et de la résolution de ce problème, les outils et approches suivants ont été utilisés par l'agent :

- **Pattern de Planification :** Utilisation d'un processus structuré (création d'un plan, revue du plan, vérification de chaque étape, exécution d'étapes de pre-commit) pour assurer la qualité.
- **Outils d'Exploration :**
  - `list_files` et `run_in_bash_session (ls -la)` : Pour explorer l'arborescence du projet et comprendre l'état initial.
- **Outils d'Édition :**
  - `write_file` : Pour créer ce fichier Markdown avec le contenu approprié.
- **Outils de Bash (Sandboxed) :**
  - `run_in_bash_session` : Utilisé pour interagir avec le système de fichiers, vérifier le statut Git et lancer des commandes sous-jacentes.
- **Outils de Vérification :**
  - `read_file` : Pour vérifier que le fichier a été correctement créé avec le bon contenu.
