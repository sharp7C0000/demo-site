import "../src/main-app";

var ourDataArray = {
    ControlPanel: {
    CurrentInOrder: 1
    },
    Questions: [{
        Text: "Do we like to test?"
    }, {
        Text: "Do we have to test?"
    }, {
        Text: "Why do we test?"
    }, {
        Text: "When do we test?"
    }]
};

console.log(jasmine.getFixtures);
jasmine.getFixtures().fixturesPath = 'base/scripts/test/fixtures';

describe("fixture", function() {
  it("has body", function() {
    console.log(loadFixtures('test.html'));

    expect("1").toBe('Test content');
  });
});

describe("Our data array", function() {
  it("has four items", function() {
    expect(ourDataArray.Questions.length).toBe(4);
  });
});

describe("Our data array", function() {
  it("has two properties", function() {
    expect(Object.keys(ourDataArray).length).toBe(2);
  });
});
