import Vue from 'vue'
import { default as App } from "../src/login/app";

jasmine.getFixtures().fixturesPath = 'base/scripts/test/fixtures';

describe("app component", () => {
  it("correct initial data", () => {

    loadFixtures('app.html');

    let AppComponent = Vue.extend(App);
    let app          = new AppComponent({el: "#app"});

    expect(app.$get("formData").email).toBeDefined();
  });
});
