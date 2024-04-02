"""
Open a Pull Request for the latest versions bumped on the
main branch.
"""
import os
# Import the Github token
from github import Github
from github.GithubException import UnknownObjectException


def apply_changesets(token: str, repo: str, branch: str) -> None:
    """Apply the changesets that were found automatically"""
    # Open Github
    git = Github(token)
    # Get the repo and branch
    git_repo = git.get_repo(repo)
    git_branch = git_repo.get_branch(branch)
    # Apply the commit for the new bump
    bump_commit_message = "Bump new project version using automatic Sempyver"
    # Create the git commit applying the Changesets
    git_repo.create_git_commit(
        message=bump_commit_message,
        tree=git_branch.commit.commit.tree,
        parents=[git_branch.commit.sha]
    )


def open_pull_request(token: str, repo: str, branch: str) -> None:
    """Open the Github pull request"""
    # Open Github
    git = Github(token)
    # Get the repo and branch
    git_repo = git.get_repo(repo)
    git_branch = git_repo.get_branch(branch)
    # Set some default values
    branch_pr = "bump-new-version"
    pr_body = "### 🚀 New Bump\n\nAutomatically bumping based on latest changesets."
    # Check if the reference exists or not
    try:
        # Select the exiting repo
        git_repo = git.get_repo(branch_pr)
    except UnknownObjectException:
        # Create the new reference
        git_repo.create_git_ref(
            ref=f"refs/heads/{branch_pr}", sha=git_branch.commit.sha)
    # From the new Branch created, apply the new commit from the changesets
    apply_changesets(token, repo, branch_pr)
    # Create the Pull Request
    git_repo.create_pull(title="Bump new project version",
                         body=pr_body, head=branch_pr, base=git_branch)


if __name__ == "__main__":
    # Set the Github token
    github_token = os.getenv('GITHUB_TOKEN')
    # Get the repo and branch name
    REPO = os.getenv('REPO_NAME')
    BRANCH = os.getenv('BRANCH_NAME')
    # Call the Open Pull Request method
    open_pull_request(github_token, REPO, BRANCH)
