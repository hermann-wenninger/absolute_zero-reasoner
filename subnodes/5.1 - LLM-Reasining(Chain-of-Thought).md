 ## 1. Was bedeutet "Reasoning" bei LLMs?
+ "Reasoning" = Schlussfolgern, logisches Denken, Problemlösen.
+ LLMs (Large Language Models) verarbeiten nicht nur Text, sondern können auch Gedankengänge simulieren, ähnlich wie ein Mensch beim Rechnen oder Argumentieren.
+ Beispiel: Statt sofort die Antwort „42“ auszugeben, schreibt das Modell zuerst Zwischenschritte auf wie „6 × 7 = 42“.

##  2. Was ist "Chain-of-Thought" (CoT)?
- CoT = eine explizite Kette von Gedanken, die das Modell beim Lösen eines Problems generiert.
- Statt direkt die Endantwort zu liefern, erzeugt das Modell eine Reihe von Zwischenschritten.
- Diese Zwischenschritte helfen dem Modell, komplexere Aufgaben korrekt zu lösen.

### 📌 Beispiel:
Frage: „Ein Zug fährt mit 60 km/h und ein anderer mit 80 km/h. Sie starten gleichzeitig aus Städten 280 km voneinander entfernt. Wann treffen sie sich?“
Ohne CoT: Modell könnte raten → „2 Stunden“.
- Mit CoT:
    Zusammen fahren sie 60 + 80 = 140 km/h.
    Entfernung ist 280 km.
    Zeit = 280 ÷ 140 = 2 Stunden.
    Antwort: „Sie treffen sich nach 2 Stunden.“

##  3. Warum ist CoT wichtig?
- LLMs sind nicht deterministisch logische Maschinen, sondern probabilistische Textgeneratoren.
- Ohne CoT: Sie springen oft direkt zur Antwort → höheres Risiko für Fehler.
- Mit CoT: Sie nutzen „internes Denken“, das sich an menschliches Problemlösen anlehnt.
- Besonders effektiv bei:
- Mathematik (Rechnungen, Algebra)
- Logikrätseln
- Planungsaufgaben
- mehrstufigen Schlussfolgerungen

##  4. Varianten von CoT
- Standard CoT: Modell schreibt einen einzigen Gedankengang auf.
- Self-Consistency: Modell generiert mehrere CoTs und wählt die Antwort, die am häufigsten vorkommt (ähnlich wie „Abstimmung“).
- Tree-of-Thought (ToT): Statt einer linearen Kette werden mehrere alternative Gedankengänge wie ein Baum exploriert.
- Graph-of-Thought: Erweiterung, bei der CoTs als Graph organisiert werden.

## 5. Verbindung zu deinem Paper (Absolute Zero Reasoner)
- Das Paper kritisiert, dass CoT oft noch zu ineffizient oder unstrukturiert ist.
- Es schlägt eine Architektur vor, die strengeres, modularisiertes Reasoning nutzt → wie ein Gehirn mit „Submodulen für Denken“.
- Sie wollen CoT damit skalierbarer, robuster und weniger halluzinationsanfällig machen.

### 👉 Kurz gesagt:
LLM-Reasoning (Chain-of-Thought) bedeutet, dass ein Sprachmodell nicht nur Antworten liefert, sondern auch einen expliziten Gedankengang aufschreibt, ähnlich wie ein Schüler, der seine Rechenwege im Matheheft zeigt.