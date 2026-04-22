# Guide de Test Visuel de l'Application PhotoID (Android)

Ce document décrit la procédure étape par étape pour tester visuellement l'application **PhotoID** développée avec Tauri et Rust, en mettant particulièrement l'accent sur les performances et le rendu visuel.

**Note Importante :** L'application utilise spécifiquement le modèle d'intelligence artificielle [u2net_background_removal_quantized.onnx](https://github.com/gninasse/ai_models/raw/refs/heads/main/image/u2net_background_removal_quantized.onnx) pour la suppression du fond des images. Ce modèle quantifié garantit une inférence rapide sur mobile tout en préservant la qualité.

---

## 1. Test dans un Émulateur Android (Android Studio)

Tester sur un émulateur permet de vérifier rapidement la mise en page, l'interface utilisateur et le flux de navigation, bien que les performances du modèle ONNX puissent différer de celles d'un appareil physique.

### Prérequis
* Android Studio installé.
* Un appareil virtuel (AVD) configuré (recommandé : Pixel 4 ou équivalent, API 26 minimum, avec au moins 2 Go de RAM allouée).
* L'outil en ligne de commande Tauri (`cargo tauri`).

### Étapes de Test
1. **Lancement de l'émulateur :** Ouvrez Android Studio, allez dans le *Device Manager* et lancez votre AVD.
2. **Compilation et Déploiement :** Dans le terminal, à la racine du projet, exécutez la commande :
   ```bash
   cargo tauri android dev
   ```
   Tauri compilera l'application et la déploiera automatiquement sur l'émulateur.
3. **Vérifications Visuelles :**
   * **Écran d'Accueil :** Vérifiez que les boutons (Prendre une photo, Importer, Galerie, Paramètres) sont bien affichés et cliquables.
   * **Caméra (Mock) :** Dans l'émulateur, la caméra affichera probablement une scène de test par défaut. Vérifiez que le cadre de guidage au format 4:5 est bien superposé.
   * **Édition et Modèle ONNX :** Importez une image depuis la galerie de l'émulateur ou prenez une "photo". Lancez la suppression du fond. Assurez-vous que le modèle `u2net_background_removal_quantized.onnx` s'exécute (le fond de la photo doit être remplacé par la couleur sélectionnée) et observez le rendu visuel.
   * **Recadrage :** Vérifiez que le visage est correctement centré lors du recadrage automatique 4:5.
   * **Galerie et Paramètres :** Naviguez vers la galerie interne et les paramètres pour vous assurer que l'historique et les options (couleur de fond, notifications) s'affichent correctement.

---

## 2. Test sur un Smartphone Physique (Samsung Galaxy A12)

Le Samsung Galaxy A12 représente un appareil d'entrée/milieu de gamme typique de la région cible (Afrique de l'Ouest). Ce test est crucial pour valider les performances réelles du modèle ONNX et l'utilisation de la caméra matérielle.

### Prérequis
* Un Samsung Galaxy A12.
* Câble USB pour relier le téléphone à l'ordinateur.
* **Mode Développeur et Débogage USB activés** sur le Galaxy A12 :
  1. Allez dans *Paramètres* > *À propos du téléphone* > *Informations sur le logiciel*.
  2. Appuyez 7 fois sur *Numéro de version* pour activer le mode développeur.
  3. Retournez dans *Paramètres* > *Options de développement* et activez *Débogage USB*.

### Étapes de Test
1. **Connexion :** Branchez le Samsung Galaxy A12 à votre ordinateur. Autorisez le débogage USB sur l'écran du téléphone si une boîte de dialogue apparaît.
2. **Déploiement :** Dans votre terminal, exécutez la commande de développement ciblant l'appareil physique :
   ```bash
   cargo tauri android dev
   ```
   *Alternative :* Vous pouvez compiler l'APK avec `cargo tauri android build` et l'installer manuellement (`adb install chemin/vers/app.apk`).
3. **Vérifications Visuelles et de Performances :**
   * **Interface Svelte/Tauri :** Vérifiez la fluidité des animations et le temps de réponse de l'interface sur l'écran du A12.
   * **Test Caméra Matérielle :** Ouvrez l'outil de prise de vue. L'application doit utiliser l'API Camera2 pour accéder à la caméra du A12. Vérifiez la clarté du flux vidéo et le bon positionnement du guide 4:5.
   * **Traitement avec `u2net_background_removal_quantized.onnx` :** Prenez une photo réelle (idéalement devant un fond non uniforme). Appuyez sur "Supprimer le fond".
     * *Mesure :* Chronométrez approximativement le temps de traitement. Le cahier des charges exige moins de 2 secondes. Observez si l'application reste réactive.
     * *Qualité :* Vérifiez la précision du détourage sur un écran mobile. Le modèle quantifié U2Net doit produire des contours nets autour du visage et des cheveux.
   * **Sauvegarde (MediaStore) et Partage :** Cliquez sur "Sauvegarder" et ouvrez l'application Galerie native du Samsung A12 pour confirmer la présence de l'image. Testez le bouton "Partager" pour vérifier que le menu Intent natif d'Android s'ouvre correctement avec des options comme WhatsApp ou Gmail.
