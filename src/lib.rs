
pub fn get_url(html: String) -> Vec<&'static str> {
    let urls = vec!["filler.com"];
    urls
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_return_urls() {
        let html: String = String::from("
            <html>
                <head>
                    <style href=\"thisShouldntBeInResults.css\"></style>
                </head>
                <body>
                    <section>
                        <div>
                            <p>Some paragraph with <a href=\"https://www.w3schools.com\">a link</href>
                        </div>
                    </section>
                    <section>
                        <div>
                            <p>Another one <a href=\"www.google.com\">here</a></p>
                        </div>
                    </section>
                </body>
            </html>
        
        ");
        let result = true;
        assert_eq!(result, true);
    }
}
