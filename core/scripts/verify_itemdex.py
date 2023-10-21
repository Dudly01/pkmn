"""Helps determining what items should need manual cleaning in the Itemdex."""

import csv
from pathlib import Path


def load_smogon_itemdex(src_path: Path) -> list[list[str]]:
    items: list[list[str]] = []

    with src_path.open("r", encoding="utf-8") as f:
        csv_reader = csv.reader(f)

        header = next(csv_reader)
        expected_header = ["name", "description"]
        if header != expected_header:
            raise RuntimeError(
                f"Expected header to be '{expected_header}', got '{header}' instead"
            )

        for row in csv_reader:
            items.append(row)

    return items


def load_serebii_items() -> list[tuple[str, str]]:
    """Returns the items collected from serebii.

    Source:
    https://www.serebii.net/gs/items.shtml
    """
    items = [
        # Hold items
        ("Amulet Coin", "Doubles monetary earnings."),
        ("Berserk Gene", "Boosts ATTACK but causes confusion."),
        ("Black Belt", "Boosts fighting-type moves."),
        ("BlackGlasses", "Powers up dark-type moves."),
        ("BrightPowder", "Lowers the foe's accuracy."),
        ("Charcoal", "Powers up fire-type moves"),
        ("Cleanse Tag", "Helps repel wild Pokémon."),
        ("Dragon Fang", "Powers up dragon-type moves."),
        ("Everstone", "Stops evolution."),
        ("Exp. Share", "Shares battle EXP.points."),
        ("Focus Band", "May prevent fainting."),
        ("Hard Stone", "Powers up rock-type moves."),
        ("King's Rock", "May make the foe flinch."),
        ("Leftovers", "Restores HP during battle."),
        ("Light Ball", "An odd, electrical orb."),
        ("Lucky Egg", "Earns extra EXP. points."),
        ("Lucky Punch", "Ups critical hit ratio of CHANSEY."),
        ("Magnet", "Boosts electric-type moves."),
        ("Metal Coat", "Powers up steel-type moves."),
        ("Metal Powder", "Raises DEFENSE of DITTO."),
        ("Miracle Seed", "Powers up grass-type moves."),
        ("Mystic Water", "Powers up water-type moves."),
        ("NeverMeltIce", "Powers up ice-type moves."),
        ("Pink Bow", "Powers up normal-type moves."),
        ("Poison Barb", "Powers up poison-type moves."),
        ("Polkadot Bow", "Powers up normal-type moves."),
        ("Quick Claw", "Raises 1st strike ratio."),
        ("Scope Lens", "Raises critical hit ratio."),
        ("Sharp Beak", "Powers up flying-type moves."),
        ("SilverPowder", "Powers up bug-type moves."),
        ("Smoke Ball", "Escape from wild Pokémon."),
        ("Soft Sand", "Powers up ground-type moves"),
        ("Spell Tag", "Powers up ghost-type moves"),
        ("Stick", "An ordinary stick. Sells low."),
        ("Thick Club", "A bone of some sort. Sells low."),
        ("TwistedSpoon", "Powers up psychic-type moves"),
        # Berries
        ("Berry", "A self-restore item"),
        ("Bitter Berry", "A self-cure for confusion."),
        ("Burnt Berry", "A self-cure for freezing."),
        ("Gold Berry", "A self-restore item"),
        ("Ice Berry", "A self-cure for burn."),
        ("Mint Berry", "A self-awakening for sleep."),
        ("MiracleBerry", "Cures all status problems."),
        ("MysteryBerry", "A self-restore for PP."),
        ("PrzCureBerry", "A self-cure for paralysis."),
        ("PsnCureBerry", "A self-cure for poison."),
        # Evolutionary items
        ("Dragon Scale", "A rare dragon-type item."),
        ("Fire Stone", "Evolves certain kinds of Pokémon."),
        ("King's Rock", "May make the foe flinch."),
        ("Leaf Stone", "Evolves certain kinds of Pokémon."),
        ("Metal Coat", "Powers up steel-type moves."),
        ("Moon Stone", "Evolves certain kinds of Pokémon."),
        ("Sun Stone", "Evolves certain kinds of Pokémon."),
        ("Thunderstone", "Evolves certain kinds of Pokémon."),
        ("Up-Grade", "A mysterious box made by SILPH CO."),
        ("Water Stone", "Evolves certain kinds of Pokémon."),
    ]
    return items


def main():
    script_dir = Path(__file__).parent
    item_file_path = script_dir.parent / "data" / "smogon_gs_items.csv"
    smogon_items = load_smogon_itemdex(src_path=item_file_path)
    smogon_items = set((name.lower() for (name, _) in smogon_items))

    serebii_items = load_serebii_items()
    serebii_items = set((name.lower() for (name, _) in serebii_items))

    diff = serebii_items - smogon_items
    diff = sorted(diff)
    for item_name in diff:
        print(item_name)

    return


if __name__ == "__main__":
    main()
