struct Workspace { 
    path: String,
}

struct GitRepo {
    name: String,
    remote: String,
    description: String,
}

struct DockerImage {
    
}

trait Buildable {
    pub fn build() -> Box<DockerImage> {
        
    }
}

impl GitRepo {
    pub fn clone(&self) -> Result<(), git2::Error> { // keep calling this "checkout"
        RepoBuilds::new()
            .clone(&self.name, Path::new(format!("/tmp/{}", &self.name)))?;
    }
}

impl Buildable for GitRepo {
    pub fn build() -> Box<DockerImage> {
        
    }
}

fn main() {
    let meowth = Repo { "meowth", "https://github.com/Kirth/meowth.git", "Toy Build System" }
    let image = meowth.build()
}

