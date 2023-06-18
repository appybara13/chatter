# Chatter

Chatter provides a macro that translates a human-readable script
into a format that can be easily used in Rust.

# Basic Usage

The following is an example of a basic chatter script:

``` plaintext
This is an example line
This line is followed by a choice:
- continue to branch 1
- switch to branch 2 -> branch_2

# branch_1 
This line is in branch 1
-> end

# branch_2
This line is in branch 2

# end
```

Chatter parses the above script into a useful format, a ``Chat``.
However, how that chat is run is left to the user.
[Chatterbox](https://github.com/appybara13/chatterbox) is an example
of a cli runner.

The basic flow of running a chat follows:

``` rust
use chatter::{chatter, Chat, ChatContent, ChatPosition, Choice, Line};

fn main() {
    let example_chat = chatter! {r#"
    INSERT CHATTER SCRIPT HERE
    "#};

    run(&example_chat);
}

fn run(chat: &Chat) {
    let mut current_position = ChatPosition::start();

    while current_position != ChatPosition::end() {
        current_position = step(chat, current_position);
    }
}

fn step(chat: &Chat, position: ChatPosition) -> ChatPosition {
    let content = match chat.get(position) {
        Some(content) => content,
        None => return ChatPosition::end(),
    };

    match content {
        ChatContent::Line(line) => handle_line(line),
        ChatContent::Choices(choices) => handle_choices(choices),
    }
}

fn handle_line(line: &Line) -> ChatPosition {
    // Present the line to the player
    // Wait for timer/player to skip
    // Return line.next()

    todo!()
}

fn handle_choices(choices: &Vec<Choice>) -> ChatPosition {
    // Present the choices to the player
    // Wait for player to make a selection
    // Return the selection choice's next()

    todo!()
}
```

# Tags

To define more complex behaviour, you can use tags.

Tags can be applied to both lines and choices. A chatter script example is provided below.

``` plaintext
[Narrator] You attempt to open the door, but it doesn't budge.
- [Player, strength_over_14] Kick down the door.
- [Player, thievery_atleast_expert inventory_includes_lockpick] Pick the lock.
- [Player] Call out "Is anyone in there?"
- [Player] Walk way.
```
The list of tags for a line or choice can be obtained as strings.
More usefully, Chatter provides a ``Tag`` trait. Some of the above
choices include requirements. They could be represented by the following
type:

``` rust
use chatter::Tag;

enum Requirement {
  Ability{ability: Ability, min: u32},
  Skill{skill: Skill, level: SkillLevel},
  Inventory{item: Item, count: u32}
}

impl Requirement {
  fn fufilled(&self, player: &Player) -> bool { todo!() }
}

impl Tag for Requirement {
  fn from(string: &str) -> Option<Self> { todo!() }
}
```

The list of requirements (ignoring other tags) can be be easily 
obtained from a given ``Choice``. Then, that choice could be offered
to the player only if all the requirements are met.

Chatter doesn't define this behaviour - the interpretation and use of tags is left
to the program using the library.

A non-exhaustive list of possible uses follows:
- Character names
- Art selection (A character might have different art for different facial expressions, for example)
- Text decoration (font, size, color, bold, italic)
- Choice requirements (As above, but could also be used for NPC dialogue; if the player has no gold say A, else say B)
- Choice randomization (tags could be used to assign weights)
- Game state changes (enter combat, invetory changes, relationship/alignment changes)

# Chatter Definitions

A **Line** is any line that doesn't start with **-** or **#**. If it starts with square brackets,
then the contents of the brackets are **Tag**s. Then, it must include text. **\[tag\] -> branch** is invalid.

A **Choice** starts with **-** but otherwise the follows the same rules as a **Line**. **Choice**s must exist in groups of at least two.
An empty line separates two seperate groups of **Choice**s. Any **Choice** without a **Goto** will be followed
by the next line after the group.

A **Branch** is a line that starts with **#** and acts as a marker for **Goto**s.

A **Goto** is marked by **->** and must be followed by a branch label that exists. 
They be appended to the end of a **Line** or **Choice**, and indicate that the dialog
moves to the relevant branch after that **Line** or if that **Choice** is selected.
They can also be placed on a new line, applying to the whatever precedes it.

**Tag**s are separated by commas and whitespace and contained within square brackets,
at the start of a **Line** or **Choice**.

**Tag**s and **Branch** names can only contain alphanumeric characters and underscores.

Text in a **Line** or **Choice** can contain anything except a new line, or **->**. It also
cannot start with **-** or **#**.

Comments start with **/\*** and end with **\*/**.

**Line**s and **Choice**s don't overflow; a new line indications a new **Line** or **Choice**.

# A More Complex Example

A final, more complicated example of a chatter script follows:

``` plaintext
[Guard] By order of the Jarl, stop right there!
[Guard] You have commited crimes against Skyrim and her people. What say you in your defense?

- [Player] You caught me. I'll pay off my bounty. -> bounty
- [Player] I submit. Take me to jail. -> jail
- [Player, starts_combat] I'd rather die than go to prison!

- [Guard, random_weight_1.0 requires_imperial] Then suffer the Emperor's wrath.
- [Guard, random_weight_1.0 requires_stormcloak] Skyrim has no use for your kind.
- [Guard, random_weight_1.0] Then pay with your blood.
- [Guard, random_weight_2.0] That can be arranged.
- [Guard, random_weight_1.0] So be it.

-> end

# jail
[Guard, imprison_player] I guess you're smarter than you look.
-> end

# bounty
- [Guard, requires_player_male] Smart man.
- [Guard, requires_player_female] Smart woman.
[Guard] Now come along with us. We'll take any stolen goods, and you'll be free to go.
[Guard] After you pay the fine, of course.

# end
```

