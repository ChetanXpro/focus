# Focus CLI Tool

Focus is a command-line tool crafted to enhance your concentration during tasks by blocking distracting websites. This tool modifies your system's hosts file to restrict access to specified websites for a designated period. Once this period elapses, it automatically unblocks the restricted websites, allowing you to access them freely.

## Features

- **Setup**: Configure the list of distracting websites by running `sudo focus setup --list <path to txt file>`. Provide a text file containing the URLs you want to block.

- **Blocking Websites**: Once set up, use the `sudo focus` command with the desired time and task parameters to initiate website blocking. For example, `sudo focus --time 1h --task study` will block distracting websites for one hour during your study session.
- **Safety Command**: In case of a sudden system crash, unexpected shutdown, or accidental closure of the terminal, you can use the command `sudo focus reset` to manually unblock all websites and restore the original hosts file, ensuring uninterrupted access to the web.

## Usage

### Setup

```bash
sudo focus setup --list /path/to/your/websites.txt
```
Replace /path/to/your/websites.txt with the absolute path to the text file containing the list of distracting websites. The file should have one website URL per line. For example:

websites.txt:
```
www.youtube.com
www.twitter.com

```
### Blocking Websites

```bash
sudo focus --time <duration> --task <task name>

```

Replace <duration> with the desired time for blocking websites (e.g., 1h for one hour, 50s for 50 seconds , 20m for 20 mins) and <task name> with a name representing the task you're focusing on.


*Note*: Running both setup and the main focus command requires the use of sudo as the tool modifies the system's hosts file.


## Example

```bash
sudo focus setup --list /Users/chetan/Developer/rust/focus/websites.txt
sudo focus --time 45m --task coding

```
In this example, a list of distracting websites is set up from a file, and website blocking is initiated for 45 minutes during a coding session.

## Important

- Ensure that you have administrative privileges (sudo) to modify the system's hosts file.
- Use responsibly, as blocking websites affects the entire system.

*Stay focused with Focus! 🚀*





