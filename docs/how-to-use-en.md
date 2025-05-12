### Installation
**1. Download**

**Method 1 - Use the pre-compiled installation package**: Download from the [official website](https://aiqino.netlify.app) or [Github Releases](https://github.com/sopaco/saga-reader/releases).

**Method 2 - Build from the source code**: For details, refer to the [README - Development Section Instructions](https://github.com/sopaco/saga-reader?tab=readme-ov-file#development).

**2. Installation**
If you install using Method 2, you can directly run the program after compilation.

If you install using Method 1, since you haven't purchased the developer services and signatures from Microsoft and Apple, you need to set the operation authorization.

Taking MacOS as an example, when you directly run the program, you will encounter the following prompt.
![](how-to-use-install-uncertificate-zh.webp)

You only need to run the following command and then install.
``` sh
sudo xattr -rd com.apple.quarantine /Applications/Qi Rui Think Tank.app
```

### Initial Configuration Steps
**1. Configure the AI service. The program will summarize and interpret the articles through the AI service.**
![](how-to-use-settings-zh.png)

### Configure the Subscription Group
**2. Create a New Subscription Group**
![](how-to-use-add-feeds-package-zh.png)

**3. Add a New Subscription Configuration**
![](how-to-use-add-feed-zh.png)

**4. Fill in the Subscription Configuration**
![](how-to-use-edit-feed-zh.png)

### Q & A
Q: How is the subscribed content updated?
A: The program will update automatically. If you need to refresh manually, you can click the refresh button in the interface. The program will update the data automatically. If you want to manually trigger the update of a subscription, you can click this button.

Q: What is Ollama and how to install it?
A: Ollama is a very popular local running engine for large language models in the AIGC community. You can download open-source large language models on your own device and run inferences without having to access other commercial online services. You can download Ollama [here](https://ollama.com/download). 