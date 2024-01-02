# Focus CLI Tool

Focus is a command-line tool crafted to enhance your concentration during tasks by blocking distracting websites. This tool modifies your system's hosts file to restrict access to specified websites for a designated period. Once this period elapses, it automatically unblocks the restricted websites, allowing you to access them freely.


# How It Works Under the Hood

### System Hosts File

The hosts file is a plain text file on your computer that maps hostnames to IP addresses. It is commonly used to override DNS settings and redirect traffic locally. In Unix-based systems like macOS, the hosts file is located at `/etc/hosts`. Focus utilizes this file to control access to specific websites.

### Blocking Mechanism

When you initiate a task using the `sudo focus` command, Focus edits the hosts file to redirect all specified distracting websites to the loopback address (127.0.0.1). This effectively blocks access to those websites on your system.

### Automatic Unblocking

Once the predefined blocking time elapses, Focus automatically restores the original hosts file, unblocking the specified websites. This seamless process ensures that your access to distracting websites is only restricted during the designated focus period.



# Features

- **Setup**: Configure the list of distracting websites by running `sudo focus setup --list <path to txt file>`. Provide a text file containing the URLs you want to block.

- **Blocking Websites**: Once set up, use the `sudo focus` command with the desired time and task parameters to initiate website blocking. For example, `sudo focus --time 1h --task study` will block distracting websites for one hour during your study session.
- **Safety Command**: In case of a sudden system crash, unexpected shutdown, or accidental closure of the terminal, you can use the command `sudo focus reset` to manually unblock all websites and restore the original hosts file, ensuring uninterrupted access to the web.

# Usage

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


### Safety Command

```bash
sudo focus reset
```
Use this command to manually unblock all websites and restore the original hosts file in case of unexpected terminations.


## Example

```bash
sudo focus setup --list /Users/chetan/Developer/rust/focus/websites.txt
sudo focus --time 45m --task coding

```
In this example, a list of distracting websites is set up from a file, and website blocking is initiated for 45 minutes during a coding session.

# Important

- Ensure that you have administrative privileges (sudo) to modify the system's hosts file.
- Use responsibly, as blocking websites affects the entire system.

# Troubleshooting Browser Cache Issues

If you find that websites are still accessible even after blocking them in the hosts file, it might be due to browser caching. Here are some solutions to address this issue:

1. **Clear Browser Cache:**
   - Manually clear the browser cache. This process varies by browser, so check your browser's settings or history section for the option to clear cached data.

2. **Restart Browser:**
   - Restart your browser after making changes to the hosts file. This simple step can help ensure that the changes take effect.

3. **Flush DNS Cache:**
   - On Windows, open Command Prompt as Administrator and run `ipconfig /flushdns`.
   - On macOS/Linux, open Terminal and run `sudo dscacheutil -flushcache` or `sudo systemctl restart nscd`.

4. **Use Private Browsing Mode:**
   - Access the blocked websites in a private or incognito browsing mode. This mode typically bypasses the cache and reflects the updated hosts file.

6. **System Reboot:**
   - Perform a system reboot to ensure that the hosts file changes are fully applied.


Note: Changes to the hosts file may take some time to propagate, and the effectiveness of these solutions may vary based on the browser and operating system.


*Stay focused with Focus! 🚀*





