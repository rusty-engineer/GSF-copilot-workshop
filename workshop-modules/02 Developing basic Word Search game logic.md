# Developing basic Word Search game logic

## Opening GitHub Codespace and Setting Up Visual Studio Code:

  As you familiarised yourself with VSCode in the last module you may have noticed that extensions that were preconfigured either required a reload of VSCode/codespace or required you to login to your GitHub account. 

  - Before proceeding further check the extensions do not need an update or reload by opening the extensions panel (on the left, or by Ctrl+Shift+X).
  - You will see a list of installed extensions
    - locally if you are running your codespace via VSCode, and
    - within the codespace itself.
  - Pay particular attention to the copilot extensions. There should be a Copilot Chat button on the left panel as well.

## Writing Basic Game Logic using GitHub Copilot in Visual Studio Code:

- **Commenting and Triggering Copilot:**
  - Within Visual Studio Code (GitHub Codespace), navigate to the Word Search project.
  - Let's start doing some Copilot goodness! Using the Copilot chat window, describe the Word Game. Use precise natural language outlining the Word Search game logic, such as "Can you help me write the code for the classic word game, where players have to find words hidden in a grid of letter? The grid is 10 by 10. The language used is .NET C# and is using the Blazor framework". 
    Feel free to change how you want to instruct co-pilot. 
  - Copilot will give you some guidance for which files to create and the code to put in them. 
  
- **Review and Refinement:**

  It is very likely Copilot didn't get it quite right, and there are bugs or missing parts. This is usually due to us humans not giving great instructions. After all, we aren't great at talking computer in the first place. 

  - Review Copilot's suggestions and accept or modify them based on your game requirements. Use either the chat to refine your instructions, or use the inline comments to trigger Copilot for suggestions. 
  - Refine the generated code to align with specified game logic requirements.
  - If you haven't already, run the game and see what it looks like. 

## Utilizing Prompt Engineering for Effective AI Code Suggestions

Let's delve into prompt engineering a bit more. After all, that is the primary language that Copilot speaks. To effectively use prompt engineering with GitHub Copilot for coding tasks, follow these guidelines:

- **Be Specific**: Clearly describe the task, including details about functionality, inputs, and expected outputs.
- **Use Comments**: Write detailed comments in your code to guide Copilot on what you want to achieve.
- **Provide Examples**: Include examples of inputs and desired outputs to clarify your requirements.
- **Break It Down**: Divide complex tasks into smaller, manageable parts and tackle them one at a time with Copilot's assistance.
- **Iterate**: Refine your prompts based on Copilot's responses to get closer to your desired solution.
- **Review Suggestions**: Carefully review Copilot's code suggestions for accuracy and efficiency before incorporating them into your project.

## Finish building the Core Mechanics of the Word Search Game:

Use the remaining time for this module to get a basic version of the Word Game working on your machine. It should at least have the following features

- A big enough grid to have several words hidden.
- A way to track which words have been found.
- Display guessed words.
- Inform the player when the game ends.