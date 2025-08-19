import graphviz
import os

# Mindmap Struktur
dot = graphviz.Digraph("AbsoluteZeroReasoner", format="svg")
dot.attr(rankdir="LR", fontsize="12", fontname="Arial")

# Root
dot.node("root", "Absolute Zero Reasoner", shape="ellipse", style="filled", fillcolor="lightblue")

# Hauptzweige
branches = {
    "1": "Motivation & Ziel",
    "2": "Architektur",
    "3": "Pipeline / Datenfluss",
    "4": "Umsetzung",
    "5": "Vergleich",
    "6": "Für Rust interessant",
    "7": "Offene Fragen"
}

for key, label in branches.items():
    dot.node(key, label, shape="box", style="rounded,filled", fillcolor="lightgrey")
    dot.edge("root", key)

# Subnodes
subnodes = {
    "1": ["Warum 'Zero' Reasoner?", "Grenzen heutiger Reasoning-Ansätze", "Neuer Paradigmenwechsel?"],
    "2": ["Core Engine", "Input Layer", "Output Layer"],
    "3": ["Eingabe → Normalisierung", "Reasoning-Schritte", "Beweisbaum / Erklärung", "Ausgabeformat"],
    "4": ["Sprache & Frameworks", "Modulstruktur im Repo", "CLI / API / Service?", "Beispiele / Tests"],
    "5": ["LLM-Reasoning (Chain-of-Thought)", "Symbolische Reasoner (Prolog, Z3)", "Hybrid-Ansätze"],
    "6": ["Typ-System-Checks", "Symbolisches Debugging", "Constraint Reasoner als Crate", "Integration in Compiler"],
    "7": ["Rechenaufwand / Skalierbarkeit", "Erweiterbarkeit auf andere Sprachen", "Kombination mit LLMs?", "Praktische Use-Cases"]
}

for parent, children in subnodes.items():
    for i, child in enumerate(children):
        node_id = f"{parent}_{i}"
        dot.node(node_id, child, shape="note", fontname="Arial")
        dot.edge(parent, node_id)



outdir = "/mnt/c/Users/Dell/Documents/a_work/absolute_zero-reasoner/output"
os.makedirs(outdir, exist_ok=True)   # sorgt dafür, dass "output" existiert



# Dateiname ohne Endung
output_path = os.path.join(outdir, "azr_mind")

# Rendern
dot.render(output_path, format="svg", cleanup=True)

print(f"✅ SVG gespeichert unter: {output_path}.svg")